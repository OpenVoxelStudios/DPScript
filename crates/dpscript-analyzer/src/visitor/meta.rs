use crate::cx::VisitCx;
use dpscript_ast::prelude::{meta::DefMeta, types::TypeRef};

pub trait MetaVisitor<'a, 'visit> {
    fn visit_meta(&mut self, _cx: &mut VisitCx<'a, 'visit>, _meta: &mut DefMeta<'a>) {}

    fn visit_type(&mut self, _cx: &mut VisitCx<'a, 'visit>, _ty: &mut TypeRef<'a>) {}
}
