use serde::Deserialize;
use std::collections::HashMap;

pub type CaptureMapping = HashMap<String, String>;

#[derive(Debug, Clone, Deserialize, serde::Serialize, Default, PartialEq, Eq)]
pub struct QueryTypeMappings {
    #[serde(default)]
    pub highlights: CaptureMapping,
    #[serde(default)]
    pub locals: CaptureMapping,
    #[serde(default)]
    pub folds: CaptureMapping,
}

pub type CaptureMappings = HashMap<String, QueryTypeMappings>;

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct HighlightItem {
    #[serde(flatten)]
    pub source: HighlightSource,
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum HighlightSource {
    Path { path: String },
    Query { query: String },
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct TreeSitterSettings {
    #[serde(rename = "captureMappings", default)]
    pub capture_mappings: CaptureMappings,
}

// Domain types that were previously in domain::settings
// These are internal representations used throughout the application

/// Query source definitions used across the domain.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QuerySource {
    Path(String),
    Inline(String),
}

impl QuerySource {
    pub fn path<P: Into<String>>(path: P) -> Self {
        Self::Path(path.into())
    }

    pub fn inline<Q: Into<String>>(query: Q) -> Self {
        Self::Inline(query.into())
    }
}

/// Per-language Tree-sitter language configuration surfaced to the domain.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanguageSettings {
    pub library: Option<String>,
    pub filetypes: Vec<String>,
    pub highlight: Vec<QuerySource>,
    pub locals: Option<Vec<QuerySource>>,
}

impl LanguageSettings {
    pub fn new(
        library: Option<String>,
        filetypes: Vec<String>,
        highlight: Vec<QuerySource>,
        locals: Option<Vec<QuerySource>>,
    ) -> Self {
        Self {
            library,
            filetypes,
            highlight,
            locals,
        }
    }
}

/// Workspace-wide Tree-sitter configuration as required by the domain.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WorkspaceSettings {
    pub capture_mappings: CaptureMappings,
}

impl WorkspaceSettings {
    pub fn new(capture_mappings: CaptureMappings) -> Self {
        Self { capture_mappings }
    }
}
