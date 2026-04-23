use crate::{
    ParseCx,
    easy::{
        parse_arr, parse_at_block, parse_block, parse_bool, parse_call, parse_component, parse_double, parse_expr, parse_float, parse_ident, parse_if, parse_init_block, parse_inner_const, parse_int, parse_pos, parse_ref, parse_return, parse_selector, parse_str, parse_tick_block, parse_top_const, parse_var
    },
    func::parse_fn,
    import::ImportParser,
    inner::Rule,
    nbt::parse_nbt,
    ops::{parse_assign, parse_binop},
    util::next_or_die,
};
use ast::{data::SourceSpan, node::Node};
use miette::Result;
use pest::iterators::Pairs;

pub fn parse_next<'a>(cx: &mut ParseCx<'a>, pairs: &mut Pairs<'a, Rule>) -> Result<Vec<Node<'a>>> {
    let mut nodes = Vec::new();
    let pair = next_or_die(cx, pairs)?;
    let rule = pair.as_rule();
    let span = SourceSpan::from(pair.as_span().into());
    let copy = pair.clone();
    let mut inner = pair.into_inner();

    match rule {
        Rule::_attr => todo!(),

        Rule::_ident => nodes.push(Node::Literal(parse_ident(
            cx,
            span,
            &mut Pairs::single(copy),
        )?)),

        Rule::_str => nodes.push(Node::Literal(parse_str(
            cx,
            span,
            &mut Pairs::single(copy),
        )?)),

        Rule::_double => nodes.push(Node::Literal(parse_double(cx, span, copy.as_str())?)),
        Rule::_float => nodes.push(Node::Literal(parse_float(cx, span, copy.as_str())?)),
        Rule::_int => nodes.push(Node::Literal(parse_int(cx, span, copy.as_str())?)),
        Rule::_bool => nodes.push(Node::Literal(parse_bool(cx, span, &mut inner)?)),
        Rule::_arr => nodes.push(Node::Literal(parse_arr(cx, span, &mut inner)?)),
        Rule::_range => todo!(),
        Rule::_nbt => nodes.push(Node::Literal(parse_nbt(cx, span, &mut inner)?)),
        Rule::_arr_ty => todo!(),
        Rule::_arr_ty_sized => todo!(),
        Rule::_obj => todo!(),
        Rule::_var => nodes.push(Node::Variable(parse_var(cx, span, &mut inner)?)),
        Rule::_field => todo!(),
        Rule::_if => nodes.push(Node::Conditional(parse_if(cx, span, &mut inner)?)),
        Rule::_call | Rule::_call_expr => nodes.push(Node::Call(parse_call(cx, span, &mut inner)?)),
        Rule::_ref => nodes.push(Node::Ref(parse_ref(cx, span, &mut inner)?)),
        Rule::_binop => nodes.push(Node::BinaryOp(parse_binop(cx, span, &mut inner)?)),
        Rule::_assign => nodes.push(Node::BinaryOp(parse_assign(cx, span, &mut inner)?)),
        Rule::_fn => nodes.push(Node::Function(parse_fn(cx, span, &mut inner)?)),
        Rule::_init_block => nodes.push(Node::Block(parse_init_block(cx, span, &mut inner)?)),
        Rule::_tick_block => nodes.push(Node::Block(parse_tick_block(cx, span, &mut inner)?)),
        Rule::_return => nodes.push(Node::Return(parse_return(cx, span, &mut inner)?)),
        Rule::_at => nodes.push(Node::At(parse_at_block(cx, span, &mut inner)?)),
        Rule::_pos => nodes.push(Node::Special(parse_pos(cx, span, &mut inner)?)),
        Rule::_selector => nodes.push(Node::Special(parse_selector(cx, span, &mut inner)?)),
        Rule::_component => nodes.push(Node::Special(parse_component(cx, span, &mut inner)?)),
        Rule::_inner_const => nodes.push(Node::Constant(parse_inner_const(cx, span, &mut inner)?)),
        Rule::_top_const => nodes.push(Node::Constant(parse_top_const(cx, span, &mut inner)?)),
        Rule::_import => nodes.extend(ImportParser { pairs: inner, span }.parse(cx)?),

        Rule::_expr => nodes.push(parse_expr(cx, span, &mut inner)?),

        Rule::body | Rule::_top_stmt_in | Rule::_inner_stmt | Rule::_top_stmt | Rule::_block => {
            nodes.extend(parse_block(cx, span, &mut inner)?)
        }

        _ => {}
    };

    Ok(nodes)
}
