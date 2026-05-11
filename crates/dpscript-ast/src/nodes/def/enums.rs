use crate::{
    prelude::{
        SourceSpan, Spanned,
        def::DefTrait,
        meta::{DefFlags, DefMeta},
    },
    util::Name,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct Enum<'a> {
    pub name: Name<'a>,
    pub flags: Vec<DefFlags>,
    pub variants: Vec<EnumVariant<'a>>,
    pub span: SourceSpan,
    pub meta: DefMeta<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct EnumVariant<'a> {
    pub name: Name<'a>,
    pub span: SourceSpan,
    pub value: EnumValue<'a>,
    pub meta: DefMeta<'a>,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet)]
pub enum EnumValue<'a> {
    String(Spanned<&'a str>),
    Byte(Spanned<i8>),
    None,
}

impl<'a> DefTrait<'a> for Enum<'a> {
    fn with_meta(mut self, meta: DefMeta<'a>) -> Self {
        self.meta = meta;
        self
    }
}
