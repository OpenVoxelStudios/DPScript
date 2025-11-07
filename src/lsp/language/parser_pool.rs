use std::collections::HashMap;
use tree_sitter::Parser;

use super::registry::LanguageRegistry;

/// Factory for creating Tree-sitter parsers with proper language configuration
pub struct ParserFactory {
    language_registry: LanguageRegistry,
}

impl ParserFactory {
    /// Create a new ParserFactory with a reference to the language registry
    pub fn new(language_registry: LanguageRegistry) -> Self {
        Self { language_registry }
    }

    /// Create a new parser for the specified language
    pub fn create_parser(&self, language_id: &str) -> Option<Parser> {
        self.language_registry.get(language_id).and_then(|lang| {
            let mut parser = Parser::new();
            // set_language can fail if the language version is incompatible
            parser.set_language(&lang).ok()?;
            Some(parser)
        })
    }
}

/// Per-document parser pool for efficient parser reuse
pub struct DocumentParserPool {
    /// Available parsers by language ID
    available: HashMap<String, Vec<Parser>>,
    /// Factory for creating new parsers
    factory: ParserFactory,
}

impl DocumentParserPool {
    /// Create a new parser pool with the given factory
    pub fn new(factory: ParserFactory) -> Self {
        Self {
            available: HashMap::new(),
            factory,
        }
    }

    /// Acquire a parser for the specified language
    /// Returns from pool if available, otherwise creates new
    pub fn acquire(&mut self, language_id: &str) -> Option<Parser> {
        // Try to get from pool first
        if let Some(parsers) = self.available.get_mut(language_id)
            && let Some(parser) = parsers.pop()
        {
            return Some(parser);
        }

        // Create new parser if not in pool
        self.factory.create_parser(language_id)
    }

    /// Release a parser back to the pool for reuse
    pub fn release(&mut self, language_id: String, parser: Parser) {
        self.available.entry(language_id).or_default().push(parser);
    }

    /// Clear all cached parsers
    pub fn clear(&mut self) {
        self.available.clear();
    }

    /// Get the number of cached parsers for a language
    pub fn pool_size(&self, language_id: &str) -> usize {
        self.available
            .get(language_id)
            .map(|v| v.len())
            .unwrap_or(0)
    }
}
