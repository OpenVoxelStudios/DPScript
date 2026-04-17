use ast::{
    ast::AST,
    at::AtNode,
    binop::{BinaryOpNode, BinaryOperation},
    block::BlockNode,
    call::CallNode,
    cond::{ConditionalNode, ElseIfNode},
    data::{HasSpan, SourceSpan},
    func::{FuncFlags, FunctionNode},
    literal::{LiteralData, LiteralNode},
    loc::{DataLocation, Identifier},
    loops::{LoopCondition, LoopNode},
    nbt::{NbtValue, NbtValueData},
    node::Node,
    ret::ReturnNode,
    scope::{ExportType, Scope},
    special::{SpecialData, SpecialNode},
    unop::{UnaryOpNode, UnaryOperation},
    var::VarNode,
};
use miette::{Error, LabeledSpan, MietteDiagnostic, NamedSource};
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    path::PathBuf,
    rc::Rc,
    sync::Arc,
};

use crate::{
    Result,
    dpscript::{data::NodeInfo, ty::TypeRef},
    mc::{
        Command, ConcatLiteral, DataCommand, DataModifyAction, DataModifyArg, DataSource,
        ExecuteCommand, ExecuteIf, ExecuteStore, Function, Literal, OptionLiteral, ReturnCommand,
        ScoreboardCommand, ScoreboardObjectivesCommand,
        util::{call_func, exec},
    },
    util::Cursor,
};

macro_rules! cg_todo {
    (L; $($name: tt)*) => {{
        warn!("TODO: {}", stringify!($($name)*));

        Literal::Inline {
            inner: "TODO".into(),
        }
    }};

    (D; $($name: tt)*) => {{
        warn!("TODO: {}", stringify!($($name)*));

        DataModifyArg::Value {
            value: Literal::Inline {
                inner: "TODO".into(),
            },
        }
    }};

    ($s: expr; $($name: tt)*) => {{
        warn!("TODO: {}", stringify!($($name)*));
        $s
    }};

    ($($name: tt)*) => {{
        warn!("TODO: {}", stringify!($($name)*));
    }};
}

pub struct CodeGenerator<'a> {
    pub code: NamedSource<String>,
    pub out_dir: PathBuf,

    /// The current AST we are compiling.
    pub ast: Rc<RefCell<AST<'a>>>,

    /// The resolved imports.
    pub imports: HashMap<&'a str, ExportType<'a>>,

    /// A map available modules.
    pub modules: Arc<HashMap<&'a str, Rc<RefCell<AST<'a>>>>>,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(
        code: NamedSource<String>,
        out_dir: PathBuf,
        ast: Rc<RefCell<AST<'a>>>,
        imports: HashMap<&'a str, ExportType<'a>>,
        modules: Arc<HashMap<&'a str, Rc<RefCell<AST<'a>>>>>,
    ) -> Self {
        Self {
            code,
            out_dir,
            ast,
            modules,
            imports,
        }
    }

    pub fn run(self) -> Result<()> {
        let mut cx = CodegenCx::new(self.code);

        cx.begin_function(&Identifier::new(
            &self.ast.borrow().namespace,
            // FIXME: Should we do this? The data should live until codegen is done, so... /shrug
            Box::leak(
                format!(
                    "zzz/{}/funcs/_dps_global_init",
                    self.ast.borrow().module.replace("::", "/")
                )
                .into_boxed_str(),
            ),
        ));

        for (_, item) in &self.ast.borrow().scope.borrow().objectives {
            cx.command(Command::Scoreboard {
                inner: ScoreboardCommand::Objectives {
                    inner: ScoreboardObjectivesCommand::Add {
                        objective: Literal::Inline {
                            inner: item.id.into(),
                        },

                        criteria: Literal::Inline {
                            inner: item.kind.0.into(),
                        },

                        display_name: OptionLiteral::None {},
                    },
                },
            });
        }

        cx.end_function();

        // TODO: Add the init function to the init tag

        // We have to do it this way because `self.ast.nodes` doesn't get updated
        // with scopes during validation, only the `.scope.*` fields.

        for node in &self.ast.borrow().scope.borrow().blocks {
            codegen_block(node, &mut cx);
        }

        for (_, node) in &self.ast.borrow().scope.borrow().functions {
            codegen_func(node, &mut cx);
        }

        for (_, funcs) in &self.ast.borrow().scope.borrow().instance_funcs {
            for (_, node) in funcs {
                codegen_func(node, &mut cx);
            }
        }

        for (id, cmds) in cx.generated {
            let func = Function {
                always_write: true, // TODO
                commands: cmds,
                id,
            };

            func.write(&self.out_dir)?;
        }

        Ok(())
    }
}

pub struct CGStackEntry<'a> {
    pub function: Identifier<'a>,
    pub commands: Vec<Command>,
}

const COMPILER_SUPPORT_SCOREBOARD: &str = "__dpscript__.compiler.support";

pub struct CodegenCx<'a> {
    pub code: NamedSource<String>,
    pub stack: Vec<CGStackEntry<'a>>,
    pub generated: HashMap<Identifier<'a>, Vec<Command>>,
    pub item_idx: usize,
    pub scopes: Vec<Rc<RefCell<Scope<'a>>>>,

    /// The number of embedded functions that have been created.
    /// These are mainly created when macros are necessary.
    pub macros: usize,

    /// The number of generated/synthetic locals.
    pub locals: usize,

    /// The number of used score entities.
    pub scores: usize,
}

impl<'a> CodegenCx<'a> {
    pub fn new(code: NamedSource<String>) -> Self {
        Self {
            code,
            stack: Vec::new(),
            generated: HashMap::new(),
            item_idx: 0,
            scopes: Vec::new(),
            macros: 0,
            locals: 0,
            scores: 0,
        }
    }

    pub fn begin_function(&mut self, id: &Identifier<'a>) -> Identifier<'a> {
        self.stack.push(CGStackEntry {
            function: id.clone(),
            commands: Vec::new(),
        });

        id.clone()
    }

    pub fn func(&self) -> &Identifier<'a> {
        &self.stack.last().unwrap().function
    }

    pub fn func_mut(&mut self) -> &CGStackEntry<'a> {
        self.stack.last_mut().unwrap()
    }

    pub fn end_function(&mut self) {
        let out = self.stack.pop().unwrap();

        self.generated.insert(out.function, out.commands);
    }

    pub fn command(&mut self, cmd: impl Into<Command>) {
        self.stack.last_mut().unwrap().commands.push(cmd.into());
    }

    pub fn func_store(&self, func: &Identifier<'a>) -> DataLocation<'a> {
        DataLocation {
            storage: func.clone(),
            path: "".into(),
        }
    }

    pub fn alloc_temp(&mut self) -> DataLocation<'a> {
        self.item_idx += 1;

        DataLocation {
            storage: Identifier {
                namespace: "dpscript".into(),
                path: "temp".into(),
            },

            // FIXME: Should we do this? The data should live until codegen is done, so... /shrug
            path: Box::leak(format!("temp{}", self.item_idx - 1).into_boxed_str()),
        }
    }

    pub fn alloc_score(&mut self) -> String {
        self.scores += 1;

        format!("temp{}", self.scores - 1)
    }

    pub fn scope(&self) -> Rc<RefCell<Scope<'a>>> {
        Rc::clone(self.scopes.last().unwrap())
    }

    pub fn push_scope(&mut self, scope: Rc<RefCell<Scope<'a>>>) {
        self.scopes.push(scope);
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop().unwrap();
    }

    pub fn begin_macro(&mut self, parent: &Identifier<'a>) -> Identifier<'a> {
        self.macros += 1;

        // FIXME: Should we do this? The data should live until codegen is done, so... /shrug
        let id = Box::leak(format!("{}/__macro/{}", parent.path, self.macros - 1).into_boxed_str());

        let id = Identifier {
            namespace: parent.namespace.clone(),
            path: id,
        };

        self.begin_function(&id)
    }

    pub fn end_macro(&mut self) {
        self.end_function();
    }

    pub fn from_data(&self, loc: &DataLocation) -> DataModifyArg {
        DataModifyArg::From {
            source: DataSource::Storage {
                target: loc.storage.clone().into(),
            },
            source_path: loc.path.clone().into(),
        }
    }

    pub fn set_data(&mut self, loc: &DataLocation, value: DataModifyArg) {
        self.command(DataCommand::Modify {
            source: DataSource::Storage {
                target: loc.storage.clone().into(),
            },
            target_path: loc.path.clone().into(),
            action: DataModifyAction::Set { inner: value },
        });
    }

    pub fn set_data_value(&mut self, loc: &DataLocation, value: impl Into<Literal>) {
        self.set_data(
            loc,
            DataModifyArg::Value {
                value: value.into(),
            },
        );
    }

    pub fn merge_data(&mut self, loc: &DataLocation, value: DataModifyArg) {
        self.command(DataCommand::Modify {
            source: DataSource::Storage {
                target: loc.storage.clone().into(),
            },
            target_path: loc.path.clone().into(),
            action: DataModifyAction::Merge { inner: value },
        });
    }

    pub fn merge_data_value(&mut self, loc: &DataLocation, value: impl Into<Literal>) {
        self.merge_data(
            loc,
            DataModifyArg::Value {
                value: value.into(),
            },
        );
    }

    pub fn append_data(&mut self, loc: &DataLocation, value: DataModifyArg) {
        self.command(DataCommand::Modify {
            source: DataSource::Storage {
                target: loc.storage.clone().into(),
            },
            target_path: loc.path.clone().into(),
            action: DataModifyAction::Append { inner: value },
        });
    }

    pub fn append_data_value(&mut self, loc: &DataLocation, value: impl Into<Literal>) {
        self.append_data(
            loc,
            DataModifyArg::Value {
                value: value.into(),
            },
        );
    }

    pub fn call_with(&mut self, id: &Identifier, data: &DataLocation) {
        self.command(Command::FunctionWith {
            name: id.clone().into(),
            data: DataSource::Storage {
                target: data.storage.clone().into(),
            },
            path: data.path.clone().into(),
        });
    }

    pub fn call(&mut self, id: &Identifier) {
        self.command(Command::Function {
            name: id.clone().into(),
        });
    }

    pub fn alloc_local(&mut self) -> (&'a str, DataLocation<'a>) {
        self.locals += 1;

        let func = self.func();
        // FIXME: Should we do this? The data should live until codegen is done, so... /shrug
        let name = Box::leak(format!("local{}", self.locals - 1).into_boxed_str());
        let loc = self.func_store(&func).subpath("locals").subpath(&name);

        (name, loc)
    }

    pub fn synthetic_local(&mut self, value: &Node<'a>) -> &'a str {
        let (local, loc) = self.alloc_local();
        let ty = value.returns(&self.scope());

        let var = VarNode {
            is_arg: false,
            location: loc,
            name: (local, value.span()),
            span: value.span(),
            ty,
            value: Some(Box::new(value.clone())),
        };

        codegen_var(&var, self);

        self.scope().borrow_mut().locals.insert(local, Rc::new(var));

        local
    }

    pub fn declare_local(
        &mut self,
        name: &'a str,
        ty: TypeRef,
        value: DataModifyArg,
    ) -> DataLocation {
        let func = self.func().clone();
        let loc = self.func_store(&func).subpath("locals").subpath(&name);

        let var = VarNode {
            is_arg: false,
            location: loc.clone(),
            name: (name, SourceSpan::new(0, 0)),
            span: SourceSpan::new(0, 0),
            ty: Some(ty),
            value: None,
        };

        codegen_var(&var, self);

        self.scope().borrow_mut().locals.insert(name, Rc::new(var));
        self.set_data(&loc, value);

        loc
    }

    pub fn bail(&self, span: SourceSpan, err: impl AsRef<str>) -> ! {
        let err = Error::new(
            MietteDiagnostic::new(err.as_ref()).and_label(LabeledSpan::at(span, "here")),
        )
        .with_source_code(self.code.clone());

        Err::<(), _>(err).unwrap();
        unreachable!("Unwrapped but did not exit!");
    }
}

pub fn codegen<'a>(node: &Node<'a>, cx: &mut CodegenCx<'a>) {
    match node {
        Node::Constant(_) | Node::Enum(_) | Node::Field(_) | Node::Import(_) => {}

        // Ignore objectives since those are handled in the top-level initializer generator
        Node::Objective(_) => {}

        Node::Loop(it) => codegen_loop(it, cx),
        Node::Function(it) => codegen_func(it, cx),
        Node::Variable(it) => codegen_var(it, cx),
        Node::Block(it) => codegen_block(it, cx),
        Node::Conditional(it) => codegen_cond(it, cx),
        Node::Return(it) => codegen_return(it, cx),
        Node::At(it) => codegen_at_block(it, cx),

        Node::Call(it) => {
            codegen_call(it, cx);
        }

        Node::BinaryOp(it) => codegen_binop(it, cx),

        _ => cx.bail(node.span(), "Expression cannot be used as a statement!"),
    }
}

pub fn codegen_loop<'a>(node: &LoopNode<'a>, cx: &mut CodegenCx<'a>) {
    match &node.condition {
        LoopCondition::Range {
            var: _,
            var_loc,
            min,
            max,
            ..
        } => {
            let func = cx.begin_function(&node.ident);

            for item in &node.body {
                codegen(item, cx);
            }

            cx.end_function();

            for i in *min..*max {
                cx.set_data_value(&var_loc, i.to_string());
                cx.call(&func);
            }
        }

        LoopCondition::Iter { .. } => cg_todo!(LoopCondition::Iter),
        LoopCondition::While { .. } => cg_todo!(LoopCondition::While),
    }
}

pub fn codegen_branch<'a>(
    cond: &Node<'a>,
    ident: &Identifier,
    body: &Vec<Node<'a>>,
    next: &mut VecDeque<ElseIfNode<'a>>,
    cx: &mut CodegenCx<'a>,
) -> ExecuteIf {
    let temp = cx.alloc_temp();
    let cond = codegen_data(&cond, cx);
    let cond_loc = temp.subpath("condition");

    cx.set_data(&cond_loc, cond);

    let score = cx.alloc_score();

    let store = ExecuteStore::Score {
        targets: score.clone().into(),
        objective: COMPILER_SUPPORT_SCOREBOARD.into(),
    };

    let cond_true = ExecuteIf::ScoreMatches {
        target: score.into(),
        target_objective: COMPILER_SUPPORT_SCOREBOARD.into(),
        range: "1".into(),
    };

    cx.command(ExecuteCommand::StoreResult {
        inner: store,
        next: Box::new(exec().run(DataCommand::Get {
            source: DataSource::Storage {
                target: cond_loc.storage.into(),
            },
            path: cond_loc.path.into(),
            scale: "1".into(),
        })),
    });

    // Generate the branch

    let true_branch = cx.begin_function(&ident);

    for node in body {
        codegen(node, cx);
    }

    cx.end_function();

    // Call the branch

    cx.command(ExecuteCommand::If {
        condition: cond_true.clone(),
        action: Box::new(exec().run(Command::Function {
            name: true_branch.into(),
        })),
    });

    // Generate the next 'else if' branch(es)

    let false_branch = cx.begin_macro(&ident);

    if let Some(it) = next.pop_front() {
        codegen_branch(&it.condition, &it.ident, &it.body, next, cx);
    }

    cx.end_macro();

    cx.command(ExecuteCommand::Unless {
        condition: cond_true.clone(),
        action: Box::new(exec().run(Command::Function {
            name: false_branch.into(),
        })),
    });

    cond_true
}

pub fn codegen_cond<'a>(node: &ConditionalNode<'a>, cx: &mut CodegenCx<'a>) {
    let cond_true = codegen_branch(
        &node.condition,
        &node.ident,
        &node.body,
        &mut VecDeque::from(node.else_ifs.clone()),
        cx,
    );

    // Generate the overall 'else' branch

    if let Some(id) = node.else_ident {
        let false_branch = cx.begin_function(&id);

        for node in &node.else_body {
            codegen(node, cx);
        }

        cx.end_function();

        cx.command(ExecuteCommand::Unless {
            condition: cond_true.clone(),
            action: Box::new(exec().run(Command::Function {
                name: false_branch.into(),
            })),
        });
    }
}

pub fn codegen_binop<'a>(node: &BinaryOpNode<'a>, cx: &mut CodegenCx<'a>) {
    match node.op.0 {
        BinaryOperation::Assign => match &*node.lhs {
            Node::BinaryOp(_) => {
                // FIXME [!! IMPORTANT !!]
                //       This may not work, as the variable could be proxied from another binary operation.
                //       Please fix this, as the value may not end up getting *actually* updated in some scenarios!

                let lhs = codegen_data(&node.lhs, cx);

                match lhs {
                    DataModifyArg::From {
                        source,
                        source_path,
                    } => {
                        let value = codegen_data(&node.rhs, cx);

                        cx.command(DataCommand::Modify {
                            source,
                            target_path: source_path,
                            action: DataModifyAction::Set { inner: value },
                        });
                    }

                    DataModifyArg::String { .. } | DataModifyArg::Value { .. } => {
                        cx.bail(node.lhs.span(), "Invalid target for assignment operation!")
                    }
                }
            }

            Node::Literal(LiteralNode {
                data: LiteralData::Ident(it),
                span,
            }) => {
                let target = cx.scope().borrow().lookup(&it).unwrap().as_node();

                let Some(target) = target.as_variable() else {
                    cx.bail(*span, "Invalid target for assignment operation!");
                };

                let value = codegen_data(&node.rhs, cx);

                cx.set_data(&target.location, value);
            }

            _ => {
                cx.bail(node.lhs.span(), "Invalid target for assignment operation!");
            }
        },

        BinaryOperation::AddAssign
        | BinaryOperation::SubAssign
        | BinaryOperation::MulAssign
        | BinaryOperation::DivAssign
        | BinaryOperation::ModAssign
        | BinaryOperation::BitAndAssign
        | BinaryOperation::BitOrAssign
        | BinaryOperation::BitXorAssign => codegen_binop(
            &BinaryOpNode {
                lhs: node.lhs.clone(),
                op: (BinaryOperation::Assign, node.op.1),
                span: node.span,

                rhs: Box::new(Node::BinaryOp(BinaryOpNode {
                    lhs: node.lhs.clone(),
                    op: (
                        match node.op.0 {
                            BinaryOperation::AddAssign => BinaryOperation::Add,
                            BinaryOperation::SubAssign => BinaryOperation::Sub,
                            BinaryOperation::MulAssign => BinaryOperation::Mul,
                            BinaryOperation::DivAssign => BinaryOperation::Div,
                            BinaryOperation::ModAssign => BinaryOperation::Mod,
                            BinaryOperation::BitAndAssign => BinaryOperation::BitAnd,
                            BinaryOperation::BitOrAssign => BinaryOperation::BitOr,
                            BinaryOperation::BitXorAssign => BinaryOperation::BitXor,
                            _ => unreachable!(),
                        },
                        node.op.1,
                    ),
                    rhs: node.rhs.clone(),
                    span: node.span,
                })),
            },
            cx,
        ),

        _ => cx.bail(node.span, "Values cannot be statements!"),
    };
}

pub fn codegen_binop_data<'a>(node: &BinaryOpNode<'a>, cx: &mut CodegenCx<'a>) -> DataModifyArg {
    match node.op.0 {
        BinaryOperation::Add => {
            let lhs_ty = node.lhs.returns(cx.scope()).unwrap();
            let rhs_ty = node.rhs.returns(cx.scope()).unwrap();

            if lhs_ty.is_nbt() && rhs_ty.is_nbt() {
                let temp = cx.alloc_temp();
                let lhs = codegen_data(&node.lhs, cx);
                let rhs = codegen_data(&node.rhs, cx);

                cx.merge_data(&temp, lhs);
                cx.merge_data(&temp, rhs);

                cx.from_data(&temp)
            } else if lhs_ty.is_stringy() && rhs_ty.is_stringy() {
                let func = cx.func().clone();
                let temp = cx.alloc_temp();
                let lhs = codegen_data(&node.lhs, cx);
                let rhs = codegen_data(&node.rhs, cx);

                cx.set_data(&temp.subpath("p0"), lhs);
                cx.set_data(&temp.subpath("p1"), rhs);

                let inner = cx.begin_macro(&func);
                let res = cx.func_store(&inner).subpath("returns");

                cx.set_data(
                    &res,
                    DataModifyArg::Value {
                        value: Literal::Concat {
                            inner: ConcatLiteral(vec![
                                "\"".into(),
                                Literal::Macro { inner: "p0".into() },
                                Literal::Macro { inner: "p1".into() },
                                "\"".into(),
                            ]),
                        },
                    },
                );

                cx.end_macro();
                cx.call_with(&inner, &temp);
                cx.from_data(&res)
            } else {
                let span = node.span();

                let call = CallNode {
                    receiver: Box::new(Node::BinaryOp(BinaryOpNode {
                        lhs: Box::new(Node::Literal(LiteralNode {
                            data: LiteralData::Ident(cx.synthetic_local(&node.lhs)),
                            span,
                        })),
                        rhs: Box::new(Node::Literal(LiteralNode {
                            data: LiteralData::Ident("add"),
                            span,
                        })),
                        op: (BinaryOperation::Field, span),
                        span,
                    })),

                    args: vec![(&*node.rhs).clone()],
                    span: node.span(),
                };

                let value = codegen_call(&call, cx);

                cx.from_data(&value)
            }
        }

        BinaryOperation::Sub
        | BinaryOperation::Mul
        | BinaryOperation::Div
        | BinaryOperation::Mod
        | BinaryOperation::BitAnd
        | BinaryOperation::BitOr
        | BinaryOperation::BitXor => {
            let span = node.span();

            let func = match node.op.0 {
                BinaryOperation::Sub => "sub",
                BinaryOperation::Mul => "mul",
                BinaryOperation::Div => "div",
                BinaryOperation::Mod => "mod",
                BinaryOperation::BitAnd => cg_todo!("TODO"; Method -> BitAnd),
                BinaryOperation::BitOr => cg_todo!("TODO"; Method -> BitOr),
                BinaryOperation::BitXor => cg_todo!("TODO"; Method -> BitXor),
                _ => cx.bail(
                    node.span,
                    "How did we get here? Binary operation (value) type was not a possible value!",
                ),
            }
            .into();

            let call = CallNode {
                receiver: Box::new(Node::BinaryOp(BinaryOpNode {
                    lhs: Box::new(Node::Literal(LiteralNode {
                        data: LiteralData::Ident(cx.synthetic_local(&node.lhs)),
                        span,
                    })),
                    rhs: Box::new(Node::Literal(LiteralNode {
                        data: LiteralData::Ident(func),
                        span,
                    })),
                    op: (BinaryOperation::Field, span),
                    span,
                })),

                args: vec![(&*node.rhs).clone()],
                span: node.span(),
            };

            let value = codegen_call(&call, cx);

            cx.from_data(&value)
        }

        BinaryOperation::CondAnd => cg_todo!(D; BinaryOperation::CondAnd),
        BinaryOperation::CondOr => cg_todo!(D; BinaryOperation::CondOr),
        BinaryOperation::CondEq => cg_todo!(D; BinaryOperation::CondEq),
        BinaryOperation::CondNeq => cg_todo!(D; BinaryOperation::CondNeq),
        BinaryOperation::CondGt => cg_todo!(D; BinaryOperation::CondGt),
        BinaryOperation::CondGe => cg_todo!(D; BinaryOperation::CondGe),
        BinaryOperation::CondLt => cg_todo!(D; BinaryOperation::CondLt),
        BinaryOperation::CondLe => cg_todo!(D; BinaryOperation::CondLe),

        BinaryOperation::Range => cg_todo!(D; BinaryOperation::Range),

        BinaryOperation::Field => {
            // FIXME: [EXTREMELY IMPORTANT]
            //        Some sort of cleanup system so the source field gets updated after this data does,
            //        otherwise it can lead to unpredictable behavior with data not updating.

            // This system here *might* work, since fields effectively get chained instead of cloned,
            // but it's unlikely.

            let Some(field) = node
                .rhs
                .as_literal()
                .map(|it| it.data.as_ident().map(|id| (id, it.span)))
                .flatten()
            else {
                cx.bail(
                    node.rhs.span(),
                    "Invalid right-side operator for field operation!",
                );
            };

            let lhs = codegen_data(&node.lhs, cx);

            match lhs {
                DataModifyArg::From {
                    source,
                    source_path,
                } => DataModifyArg::From {
                    source: source,
                    source_path: Literal::Concat {
                        inner: ConcatLiteral(vec![source_path, field.0.into()]),
                    },
                },

                DataModifyArg::Value { .. } | DataModifyArg::String { .. } => {
                    let temp = cx.alloc_temp();
                    let output = temp.subpath(field.0);

                    cx.set_data(&temp, lhs);
                    cx.from_data(&output)
                }
            }
        }

        BinaryOperation::ArrayIndex => {
            // FIXME: [EXTREMELY IMPORTANT]
            //        Some sort of cleanup system so the source array gets updated after this reference does,
            //        otherwise it can lead to unpredictable behavior with data not updating. Right now it returns
            //        an effectively read-only reference to the element at the given index, since with dynamic
            //        variables it cannot be accessed without the use of a macro.

            let func = cx.func().clone();
            let temp = cx.alloc_temp();
            let arr = codegen_data(&node.lhs, cx);
            let index = codegen_data(&node.rhs, cx);

            let arr_loc = temp.subpath("array");

            cx.set_data(&arr_loc, arr);
            cx.set_data_value(&temp, "{}");
            cx.set_data(&temp.subpath("index"), index);

            let shim = cx.begin_macro(&func);
            let output = cx.func_store(&shim).subpath("returns");

            cx.set_data(
                &output,
                DataModifyArg::From {
                    source: DataSource::Storage {
                        target: arr_loc.storage.into(),
                    },
                    source_path: Literal::Concat {
                        inner: ConcatLiteral(vec![
                            arr_loc.path.into(),
                            "[".into(),
                            Literal::Macro {
                                inner: "index".into(),
                            },
                            "]".into(),
                        ]),
                    },
                },
            );

            cx.end_macro();
            cx.call_with(&shim, &temp);
            cx.from_data(&output)
        }

        _ => cx.bail(node.span, "Assignment operations cannot be values!"),
    }
}

pub fn codegen_at_block<'a>(block: &AtNode<'a>, cx: &mut CodegenCx<'a>) {
    let func = cx.begin_function(&block.ident);

    cx.push_scope(block.scope.clone().unwrap());

    for node in &block.body {
        codegen(node, cx);
    }

    cx.pop_scope();
    cx.end_function();

    let caller = cx.begin_macro(&func);

    cx.command(exec().at_(
        Literal::Macro {
            inner: "targets".into(),
        },
        exec().run(call_func(&func)),
    ));

    cx.end_macro();

    let data = cx.alloc_temp();

    cx.set_data_value(&data, "{}");

    let targets = codegen_data(&block.pos, cx);

    cx.set_data(&data.subpath("targets"), targets);
    cx.call_with(&caller, &data);
}

pub fn codegen_block<'a>(block: &BlockNode<'a>, cx: &mut CodegenCx<'a>) {
    let _func = cx.begin_function(&block.ident);

    cx.push_scope(block.scope.clone().unwrap());

    for node in &block.body {
        codegen(node, cx);
    }

    cx.pop_scope();
    cx.end_function();

    // TODO: Add to the proper tag
}

pub fn codegen_func<'a>(func: &FunctionNode<'a>, cx: &mut CodegenCx<'a>) {
    // TODO: Inline functions, ignore them here

    cx.begin_function(&func.ident);
    cx.push_scope(func.scope.clone().unwrap());

    if func.flags.contains(FuncFlags::Facade) {
        // Theoretically these unwrap() calls are all safe because the validator should check it all
        let attr = func
            .attrs
            .get("cmd")
            .unwrap()
            .values
            .get(0)
            .unwrap()
            .as_literal()
            .unwrap()
            .data
            .as_string()
            .unwrap();

        let mut cmd = Vec::new();
        let mut buf = String::new();
        let mut in_arg = false;
        let mut r = Cursor::<String, NamedSource<String>>::new_from_code("(attr)", &attr);
        let mut any_args = false;

        while r.has_next() {
            let ch = r.next().unwrap();
            if ch == '{' && !r.peek().is_some_and(|it| it == '{') {
                if in_arg {
                    let err = Error::new(
                        MietteDiagnostic::new("Cannot have a '{' inside of an argument specifier!")
                            .with_help("Try using '{{'?"),
                    )
                    .with_source_code(attr.to_string());

                    Err::<(), _>(err).unwrap();
                    unreachable!("Unwrapped but did not exit!");
                }

                in_arg = true;
                any_args = true;

                cmd.push(buf);
                buf = String::new();
            } else if ch == '}' && !r.peek().is_some_and(|it| it == '}') {
                if !in_arg {
                    let err = Error::new(
                        MietteDiagnostic::new(
                            "Cannot have a '}' outside of an argument specifier!",
                        )
                        .with_help("Try using '}}'?"),
                    )
                    .with_source_code(attr.to_string());

                    Err::<(), _>(err).unwrap();
                    unreachable!("Unwrapped but did not exit!");
                }

                in_arg = false;

                if buf.is_empty() {
                    let err = Error::new(
                        MietteDiagnostic::new("Argument specifier had no value!")
                            .with_help("Try putting something between the braces ('{}')?"),
                    )
                    .with_source_code(attr.to_string());

                    Err::<(), _>(err).unwrap();
                    unreachable!("Unwrapped but did not exit!");
                }

                cmd.push(format!("$({})", buf));
                buf = String::new();
            } else {
                buf.push(ch);
            }
        }

        if in_arg {
            let err = Error::new(
                MietteDiagnostic::new("Argument specifier did not close!")
                    .with_help("Try adding '}'?"),
            )
            .with_source_code(attr.to_string());

            Err::<(), _>(err).unwrap();
            unreachable!("Unwrapped but did not exit!");
        }

        if !buf.is_empty() {
            cmd.push(buf);
        }

        let cmd = format!("{}{}", if any_args { "$" } else { "" }, cmd.join(""));

        cx.command(Command::Custom { inner: cmd.into() });
    } else {
        for node in &func.body {
            codegen(node, cx);
        }
    }

    cx.pop_scope();
    cx.end_function();
}

pub fn codegen_data<'a>(var: &Node<'a>, cx: &mut CodegenCx<'a>) -> DataModifyArg {
    match var {
        Node::Variable(it) => cx.from_data(&it.location),
        Node::Constant(it) => codegen_data(&it.value, cx),
        Node::Literal(it) => codegen_literal_data(it, cx),
        Node::UnaryOp(it) => codegen_unary_data(it, cx),
        Node::Special(it) => codegen_special_data(it, cx),
        Node::BinaryOp(it) => codegen_binop_data(it, cx),

        Node::Call(it) => {
            let loc = codegen_call(it, cx);

            cx.from_data(&loc)
        }

        Node::Objective(it) => DataModifyArg::Value {
            value: format!("\"{}\"", it.id).into(),
        },

        _ => cx.bail(var.span(), "Expression cannot be a value!"),
    }
}

pub fn codegen_literal_data(it: &LiteralNode, cx: &mut CodegenCx) -> DataModifyArg {
    match &it.data {
        LiteralData::String(it) => DataModifyArg::Value {
            value: format!("\"{it}\"").into(),
        },

        LiteralData::Int(it) => DataModifyArg::Value {
            value: it.to_string().into(),
        },

        LiteralData::Float(it) => DataModifyArg::Value {
            value: it.to_string().into(),
        },

        LiteralData::Double(it) => DataModifyArg::Value {
            value: it.to_string().into(),
        },

        LiteralData::Bool(it) => DataModifyArg::Value {
            value: it.to_string().into(),
        },

        LiteralData::Ident(id) => {
            let Some(var) = cx.scope().borrow().lookup(&id) else {
                cx.bail(it.span, format!("Unable to resolve variable {}!", id));
            };

            let var = var.as_node();

            codegen_data(&var, cx)
        }

        LiteralData::Array(it) => {
            let temp = cx.alloc_temp();

            cx.set_data_value(&temp, "[]");

            for item in it {
                let value = codegen_data(item, cx);

                cx.append_data(&temp, value);
            }

            cx.from_data(&temp)
        }

        LiteralData::Nbt(it) => codegen_nbt_data(it, cx),
    }
}

pub fn codegen_special_data<'a>(it: &SpecialNode<'a>, cx: &mut CodegenCx<'a>) -> DataModifyArg {
    match &it.data {
        // FIXME: How do we access the root object? Is it actually just '.'?
        SpecialData::Selector(it) => DataModifyArg::From {
            source: DataSource::Entity { target: it.into() },
            source_path: ".".into(),
        },

        SpecialData::Pos(x, y, z) => {
            let temp = cx.alloc_temp();

            cx.set_data_value(&temp, "[]");

            let x = codegen_data(x, cx);
            let y = codegen_data(y, cx);
            let z = codegen_data(z, cx);

            cx.set_data(&temp.subpath("[0]"), x);
            cx.set_data(&temp.subpath("[1]"), y);
            cx.set_data(&temp.subpath("[2]"), z);

            cx.from_data(&temp)
        }

        SpecialData::Component(it) => codegen_nbt_data(it, cx),
    }
}

// TODO: Compile-time math
pub fn codegen_unary_data<'a>(it: &UnaryOpNode<'a>, cx: &mut CodegenCx<'a>) -> DataModifyArg {
    match it.op {
        UnaryOperation::None => codegen_data(&it.value, cx),

        UnaryOperation::Negate => {
            let (local, loc) = cx.alloc_local();
            let ty = it.value.returns(&cx.scope());

            let var = VarNode {
                is_arg: false,
                location: loc,
                name: (local, it.span),
                span: it.span,
                ty,
                value: Some(it.value.clone()),
            };

            codegen_var(&var, cx);

            cx.scope().borrow_mut().locals.insert(local, Rc::new(var));

            let span = it.span;

            let call = CallNode {
                args: vec![],
                span,

                receiver: Box::new(Node::BinaryOp(BinaryOpNode {
                    lhs: Box::new(Node::Literal(LiteralNode {
                        data: LiteralData::Ident(local),
                        span,
                    })),
                    rhs: Box::new(Node::Literal(LiteralNode {
                        data: LiteralData::Ident("negate"), // operator function for negating values
                        span,
                    })),
                    op: (BinaryOperation::Field, span),
                    span,
                })),
            };

            let value = codegen_call(&call, cx);

            cx.from_data(&value)
        }

        UnaryOperation::LocalOffset => {
            // FIXME [! IMPORTANT !]
            //       This does NOT work! You can't do this in data storage! Fix this!

            let offset = codegen_data(&it.value, cx);
            let args = cx.alloc_temp();

            cx.set_data_value(&args, "{}");
            cx.set_data(&args.subpath("offset"), offset);

            let target = cx.begin_macro(&cx.func().clone());
            let output = cx.func_store(&target).subpath("returns");

            cx.set_data_value(
                &output,
                Literal::Concat {
                    inner: ConcatLiteral(vec![
                        "~".into(),
                        Literal::Macro {
                            inner: "offset".into(),
                        },
                    ]),
                },
            );

            cx.end_macro();
            cx.call_with(&target, &args);
            cx.from_data(&output)
        }

        UnaryOperation::Invert => {
            let temp = cx.alloc_temp();
            let cond_loc = temp.subpath("cond");
            let score = cx.alloc_score();

            cx.command(ExecuteCommand::StoreResult {
                inner: ExecuteStore::Score {
                    targets: score.clone().into(),
                    objective: COMPILER_SUPPORT_SCOREBOARD.into(),
                },
                next: Box::new(exec().run(DataCommand::Get {
                    source: DataSource::Storage {
                        target: cond_loc.storage.clone().into(),
                    },
                    path: cond_loc.path.clone().into(),
                    scale: "1".into(),
                })),
            });

            cx.command(
                exec()
                    .if_()
                    .score_matches(&score, COMPILER_SUPPORT_SCOREBOARD, "1")
                    .then(exec().run(Command::Data {
                        inner: DataCommand::Modify {
                            source: DataSource::Storage {
                                target: cond_loc.storage.clone().into(),
                            },
                            target_path: cond_loc.path.clone().into(),
                            action: DataModifyAction::Set {
                                inner: DataModifyArg::Value { value: "0".into() },
                            },
                        },
                    })),
            );

            cx.command(
                exec()
                    .unless_()
                    .score_matches(&score, COMPILER_SUPPORT_SCOREBOARD, "1")
                    .then(exec().run(Command::Data {
                        inner: DataCommand::Modify {
                            source: DataSource::Storage {
                                target: cond_loc.storage.clone().into(),
                            },
                            target_path: cond_loc.path.clone().into(),
                            action: DataModifyAction::Set {
                                inner: DataModifyArg::Value { value: "1".into() },
                            },
                        },
                    })),
            );

            cx.from_data(&cond_loc)
        }

        UnaryOperation::RangeStart => cg_todo!(D; UnaryOperation::RangeStart),
        UnaryOperation::RangeEnd => cg_todo!(D; UnaryOperation::RangeEnd),
    }
}

pub fn codegen_nbt_data<'a>(it: &NbtValue<'a>, cx: &mut CodegenCx<'a>) -> DataModifyArg {
    match &it.data {
        NbtValueData::Map(it) => {
            let temp = cx.alloc_temp();

            cx.set_data_value(&temp, "{}");

            for (k, v) in it {
                let value = codegen_nbt_data(v, cx);

                cx.set_data(&temp.subpath(k), value);
            }

            cx.from_data(&temp)
        }

        NbtValueData::Array(it) => {
            let temp = cx.alloc_temp();

            cx.set_data_value(&temp, "[]");

            for item in it {
                let value = codegen_nbt_data(item, cx);

                cx.append_data(&temp, value);
            }

            cx.from_data(&temp)
        }

        NbtValueData::String(it) => DataModifyArg::Value {
            value: format!("\"{it}\"").into(),
        },

        NbtValueData::Float(it) => DataModifyArg::Value {
            value: format!("{it}f").into(),
        },

        NbtValueData::Double(it) => DataModifyArg::Value {
            value: format!("{it}d").into(),
        },

        NbtValueData::Int(it) => DataModifyArg::Value {
            value: format!("{it}").into(),
        },

        NbtValueData::Long(it) => DataModifyArg::Value {
            value: format!("{it}").into(),
        },

        NbtValueData::Bool(it) => DataModifyArg::Value {
            value: format!("{it}").into(),
        },

        NbtValueData::Byte(it) => DataModifyArg::Value {
            value: format!("{it}b").into(),
        },

        NbtValueData::Expr(it) => codegen_data(it, cx),
    }
}

pub fn codegen_var<'a>(node: &VarNode<'a>, cx: &mut CodegenCx<'a>) {
    if let Some(value) = &node.value {
        let value = codegen_data(value, cx);

        cx.set_data(&node.location, value);
    }
}

pub fn codegen_return<'a>(node: &ReturnNode<'a>, cx: &mut CodegenCx<'a>) {
    let func = cx.func();
    let ret = cx.func_store(func).subpath("returns");

    if let Some(value) = &node.value {
        let inner = codegen_data(value, cx);

        cx.command(DataCommand::Modify {
            source: DataSource::Storage {
                target: ret.storage.into(),
            },
            target_path: ret.path.into(),
            action: DataModifyAction::Set { inner },
        });
    }

    cx.command(ReturnCommand::Value { value: "0".into() });
}

pub fn codegen_call<'a>(call: &CallNode<'a>, cx: &mut CodegenCx<'a>) -> DataLocation<'a> {
    let temp = cx.alloc_temp();

    let Some(target) = call.target_fn(cx.scope()) else {
        cx.bail(call.span, format!("Missing function: {}", call.func));
    };

    let target = target.clone();
    let mut start_idx = 0;
    let args = temp.subpath("args");

    cx.command(DataCommand::Modify {
        source: DataSource::Storage {
            target: args.storage.to_string().into(),
        },

        target_path: args.path.clone().into(),

        action: DataModifyAction::Set {
            inner: DataModifyArg::Value { value: "{}".into() },
        },
    });

    // TODO: Get function name from receiver and then call that function, storing the receiver as a param
    let var = cx.scope().lookup(call.receiver).unwrap().as_node();
    let name = &target.args[0].name;
    let target_loc = temp.subpath(format!("args.{name}"));
    let inner = codegen_data(&var, cx);

    cx.command(DataCommand::Modify {
        source: DataSource::Storage {
            target: target_loc.storage.to_string().into(),
        },

        target_path: target_loc.path.into(),

        action: DataModifyAction::Set { inner },
    });

    start_idx = 1;

    for i in 0..call.args.len() {
        let name = &target.args[i + start_idx].name;
        let target = temp.subpath(format!("args.{name}"));
        let arg = &call.args[i];
        let inner = codegen_data(&arg, cx);

        cx.command(DataCommand::Modify {
            source: DataSource::Storage {
                target: target.storage.to_string().into(),
            },

            target_path: target.path.into(),

            action: DataModifyAction::Set { inner },
        });
    }

    // TODO: Inline functions, embed them here

    cx.command(Command::FunctionWith {
        name: target.ident.clone().into(),
        data: DataSource::Storage {
            target: args.storage.into(),
        },
        path: OptionLiteral::Inline {
            inner: args.path.into(),
        },
    });

    cx.func_store(&target.ident).subpath("returns")
}
