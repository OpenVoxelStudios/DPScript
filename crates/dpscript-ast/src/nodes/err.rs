use thiserror::Error;

#[derive(
    Debug, Clone, Copy, Error, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
#[error("expected variant did not match self!")]
pub struct TryIntoNodeError;
