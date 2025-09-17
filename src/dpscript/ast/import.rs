use miette::SourceSpan;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct ImportNode {
    pub span: SourceSpan,
    pub path: todo!(),
}
