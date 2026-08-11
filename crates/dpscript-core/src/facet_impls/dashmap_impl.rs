use dashmap::DashMap;
use std::{collections::HashMap, convert::Infallible, hash::Hash};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet)]
#[facet(transparent)]
#[serde(transparent)]
pub struct DashMapProxy<K: Eq + Hash, V>(HashMap<K, V>);

impl<K: Eq + Hash, V> TryFrom<DashMapProxy<K, V>> for DashMap<K, V> {
    type Error = &'static str;

    fn try_from(value: DashMapProxy<K, V>) -> Result<Self, Self::Error> {
        Ok(DashMap::from_iter(value.0.into_iter()))
    }
}

impl<K: Eq + Hash + Clone, V: Clone> TryFrom<&DashMap<K, V>> for DashMapProxy<K, V> {
    type Error = Infallible;

    fn try_from(value: &DashMap<K, V>) -> Result<Self, Self::Error> {
        Ok(DashMapProxy(HashMap::from_iter(
            value
                .iter()
                .map(|it| (it.key().clone(), it.value().clone())),
        )))
    }
}
