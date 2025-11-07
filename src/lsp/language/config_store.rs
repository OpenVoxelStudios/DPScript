use crate::lsp::config::{CaptureMappings, TreeSitterSettings};
use std::sync::RwLock;

/// Stores and manages language configurations
pub struct ConfigStore {
    capture_mappings: RwLock<CaptureMappings>,
}

impl ConfigStore {
    pub fn new() -> Self {
        Self {
            capture_mappings: RwLock::new(CaptureMappings::default()),
        }
    }

    // ========== Language Configs ==========
    pub fn update_from_settings(&self, settings: &TreeSitterSettings) {
        self.set_capture_mappings(settings.capture_mappings.clone());
    }

    // ========== Capture Mappings ==========
    pub fn set_capture_mappings(&self, mappings: CaptureMappings) {
        match self.capture_mappings.write() {
            Ok(mut guard) => *guard = mappings,
            Err(poisoned) => {
                warn!(target: "treesitter_ls::lock_recovery", "Recovered from poisoned lock in config_store::set_capture_mappings");
                *poisoned.into_inner() = mappings;
            }
        }
    }

    pub fn get_capture_mappings(&self) -> CaptureMappings {
        match self.capture_mappings.read() {
            Ok(guard) => guard.clone(),
            Err(poisoned) => {
                warn!(target: "treesitter_ls::lock_recovery", "Recovered from poisoned lock in config_store::get_capture_mappings");
                poisoned.into_inner().clone()
            }
        }
    }

    /// Clear all configurations
    pub fn clear(&self) {
        match self.capture_mappings.write() {
            Ok(mut guard) => *guard = CaptureMappings::default(),
            Err(poisoned) => {
                warn!(target: "treesitter_ls::lock_recovery", "Recovered from poisoned lock in config_store::clear (capture_mappings)");
                *poisoned.into_inner() = CaptureMappings::default();
            }
        }
    }
}

impl Default for ConfigStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_store_capture_mappings() {
        let store = ConfigStore::new();

        let mappings = CaptureMappings::default();
        store.set_capture_mappings(mappings.clone());

        let retrieved = store.get_capture_mappings();
        // Just check that we can store and retrieve mappings
        assert_eq!(retrieved.len(), mappings.len());
    }
}
