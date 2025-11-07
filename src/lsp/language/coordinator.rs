use super::config_store::ConfigStore;
use super::parser_pool::{DocumentParserPool, ParserFactory};
use super::query_store::QueryStore;
use super::registry::LanguageRegistry;
use crate::lsp::config::CaptureMappings;
use crate::lsp::language::dpscript::DPSCRIPT_LANGUAGE_ID;
use std::sync::Arc;
use tree_sitter::Language;

/// Coordinates language runtime components (registry, queries, configs).
pub struct LanguageCoordinator {
    pub(crate) query_store: QueryStore,
    pub(crate) config_store: ConfigStore,
    pub(crate) language_registry: LanguageRegistry,
}

impl Default for LanguageCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageCoordinator {
    pub fn new() -> Self {
        let language = Language::new(tree_sitter_dpscript::LANGUAGE);

        Self {
            query_store: QueryStore::new(&language),
            config_store: ConfigStore::new(),
            language_registry: LanguageRegistry::new(),
        }
    }

    /// Get language for a document path
    pub fn get_language_for_path(&self, path: &str) -> Option<String> {
        if Self::extract_extension(path) == "dps" {
            Some(DPSCRIPT_LANGUAGE_ID.into())
        } else {
            None
        }
    }

    /// Get language for a file extension
    pub fn get_language_for_extension(&self, extension: &str) -> Option<String> {
        if extension == "dps" {
            Some(DPSCRIPT_LANGUAGE_ID.into())
        } else {
            None
        }
    }

    /// Create a document parser pool
    pub fn create_document_parser_pool(&self) -> DocumentParserPool {
        let parser_factory = ParserFactory::new(self.language_registry.clone());
        DocumentParserPool::new(parser_factory)
    }

    /// Get highlight query for a language
    pub fn get_highlight_query(&self) -> Arc<tree_sitter::Query> {
        self.query_store.get_highlight_query()
    }

    /// Get locals query for a language
    pub fn get_locals_query(&self) -> Arc<tree_sitter::Query> {
        self.query_store.get_locals_query()
    }

    pub fn get_injection_query(&self) -> Arc<tree_sitter::Query> {
        self.query_store.get_injection_query()
    }

    /// Get capture mappings
    pub fn get_capture_mappings(&self) -> CaptureMappings {
        let config_mappings = self.config_store.get_capture_mappings();
        config_mappings
            .iter()
            .map(|(lang, mappings)| (lang.clone(), mappings.clone()))
            .collect::<CaptureMappings>()
    }

    /// Extract file extension from a path
    fn extract_extension(path: &str) -> &str {
        path.split('.').next_back().unwrap_or("")
    }
}
