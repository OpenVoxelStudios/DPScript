use std::sync::Arc;
use tree_sitter::{Language, Query};

/// Stores and manages Tree-sitter queries
pub struct QueryStore {
    highlight_query: Arc<Query>,
    locals_query: Arc<Query>,
    injection_query: Arc<Query>,
}

impl QueryStore {
    pub fn new(dpscript: &Language) -> Self {
        Self {
            highlight_query: Arc::new(
                Query::new(dpscript, tree_sitter_dpscript::HIGHLIGHTS_QUERY).unwrap(),
            ),

            locals_query: Arc::new(
                Query::new(dpscript, tree_sitter_dpscript::LOCALS_QUERY).unwrap(),
            ),

            injection_query: Arc::new(
                Query::new(dpscript, tree_sitter_dpscript::INJECTIONS_QUERY).unwrap(),
            ),
        }
    }

    pub fn get_highlight_query(&self) -> Arc<Query> {
        Arc::clone(&self.highlight_query)
    }

    pub fn get_locals_query(&self) -> Arc<Query> {
        Arc::clone(&self.locals_query)
    }

    pub fn get_injection_query(&self) -> Arc<Query> {
        Arc::clone(&self.injection_query)
    }
}
