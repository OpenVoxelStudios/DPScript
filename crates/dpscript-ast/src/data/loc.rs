use super::Identifier;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct DataLocation<'a> {
    pub storage: Identifier<'a>,
    pub path: &'a str,
}

impl<'a> DataLocation<'a> {
    pub fn subpath(&self, path: impl AsRef<str>) -> DataLocation<'a> {
        Self {
            storage: self.storage.clone(),

            // FIXME: Should we do this? The data should live until at least codegen is done, so... /shrug
            path: Box::leak(
                if self.path.is_empty() {
                    path.as_ref().into()
                } else if path.as_ref().starts_with("[") {
                    format!("{}{}", self.path, path.as_ref())
                } else {
                    format!("{}.{}", self.path, path.as_ref())
                }
                .into_boxed_str(),
            ),
        }
    }
}

impl<'a> fmt::Display for DataLocation<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}#{}", self.storage, self.path)
    }
}
