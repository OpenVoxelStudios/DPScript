use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Identifier<'a> {
    pub namespace: &'a str,
    pub path: &'a str,
}

impl<'a> Identifier<'a> {
    pub fn new(namespace: &'a str, path: &'a str) -> Self {
        Self { namespace, path }
    }
}

impl<'a> fmt::Display for Identifier<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}
