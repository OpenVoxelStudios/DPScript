use crate::{AddSpan, util::Spanned};
use miette::SourceSpan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    /// This would be the `std/some/` part of `import std/some/{abc, def, ghi}`.
    pub base: Vec<Spanned<String>>,
    pub imports: Vec<ImportNode>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportNode {
    Object(Spanned<String>),
    Module(Import),
}

impl Import {
    /// Get the ID of the module we are importing from
    pub fn module(&self) -> String {
        self.base
            .iter()
            .map(|(v, _)| v.clone())
            .collect::<Vec<_>>()
            .join("/")
    }

    pub fn module_span(&self) -> SourceSpan {
        let parts = self.base.clone();

        let Some(&(_, mut span)) = parts.first() else {
            panic!("Achievement Unlocked: How did we get here?")
        };

        for (_, item) in parts {
            span = span.add(item);
        }

        span
    }
}
