use crate::{
    data::NamedSource,
    node::Node,
    scope::{Scope, collect_exports},
};
use itertools::Itertools;
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct AST<'a> {
    /// The namespace of the AST.
    pub namespace: &'a str,

    /// The module's name (a::b::c)
    pub module: &'a str,

    /// The source code of the module.
    #[serde(skip)]
    pub code: NamedSource<'a>,

    /// All the nodes, including the ones above.
    pub nodes: Vec<Node<'a>>,

    /// The AST's local scope.
    #[serde(skip)]
    pub scope: Rc<RefCell<Scope<'a>>>,
}

macro_rules! only {
    ($list: ident: $type: ident) => {
        $list
            .iter()
            .filter_map(|it| match it {
                Node::$type(value) => Some(value.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
    };

    (m; $list: ident: $type: ident) => {
        $list
            .iter()
            .filter_map(|it| match it {
                Node::$type(value) => Some((value.name, value.clone())),
                _ => None,
            })
            .collect::<BTreeMap<_, _>>()
    };

    (ms; $list: ident: $type: ident) => {
        $list
            .iter()
            .filter_map(|it| match it {
                Node::$type(value) => Some((value.name.0, value.clone())),
                _ => None,
            })
            .collect::<BTreeMap<_, _>>()
    };

    (msr; $list: ident: $type: ident) => {
        $list
            .iter()
            .filter_map(|it| match it {
                Node::$type(value) => Some((value.name.0, Rc::new(value.clone()))),
                _ => None,
            })
            .collect::<BTreeMap<_, _>>()
    };
}

impl<'a> AST<'a> {
    pub fn new(
        namespace: &'a str,
        module: &'a str,
        code: NamedSource<'a>,
        nodes: Vec<Node<'a>>,
    ) -> Self {
        Self {
            namespace,
            module,
            code,

            scope: Rc::new(RefCell::new(Scope {
                name: module,
                locals: BTreeMap::new(),
                exports: collect_exports(&nodes),
                blocks: only!(nodes: Block),
                constants: only!(msr; nodes: Constant),
                enums: only!(ms; nodes: Enum),

                functions: only!(msr; nodes: Function)
                    .into_iter()
                    .filter(|it| it.1.receiver.is_none())
                    .collect(),

                imports: only!(nodes: Import),
                objectives: only!(msr; nodes: Objective),
                parents: Vec::new(),

                fields: only!(nodes: Field)
                    .into_iter()
                    .map(|it| (it.owner.0, (it.name.0, it)))
                    .into_group_map()
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            k,
                            v.into_iter()
                                .map(|(a, b)| (a, Rc::new(b)))
                                .collect::<BTreeMap<_, _>>(),
                        )
                    })
                    .collect::<BTreeMap<_, _>>(),

                // My little baby abomination... I'm so proud of it :) (*sobbing*)
                instance_funcs: only!(nodes: Function)
                    .into_iter()
                    .filter_map(|it| it.receiver.clone().map(|r| (r.0, (it.name.0, it))))
                    .into_group_map()
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            k,
                            v.into_iter()
                                .map(|(a, b)| (a, Rc::new(b)))
                                .collect::<BTreeMap<_, _>>(),
                        )
                    })
                    .collect::<BTreeMap<_, _>>(),
            })),

            nodes,
        }
    }
}
