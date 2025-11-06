use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Identifier {
    pub namespace: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DataLocation {
    pub storage: Identifier,
    pub path: String,
}

impl DataLocation {
    pub fn subpath(&self, path: impl AsRef<str>) -> DataLocation {
        Self {
            storage: self.storage.clone(),
            path: if self.path.is_empty() {
                path.as_ref().into()
            } else if path.as_ref().starts_with("[") {
                format!("{}{}", self.path, path.as_ref())
            } else {
                format!("{}.{}", self.path, path.as_ref())
            },
        }
    }
}

impl Identifier {
    pub fn new(ns: impl AsRef<str>, path: impl AsRef<str>) -> Self {
        Self {
            namespace: ns.as_ref().into(),
            path: path.as_ref().into(),
        }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

impl fmt::Display for DataLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}#{}", self.storage, self.path)
    }
}
