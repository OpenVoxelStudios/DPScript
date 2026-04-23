use crate::prelude::*;

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
            nodes,
        }
    }
}
