mod has_span;
mod ident;
mod loc;
mod source;
mod span;
mod span_ops;

pub use has_span::HasSpan;
pub use ident::Identifier;
pub use loc::DataLocation;
pub use source::NamedSource;
pub use span::SourceSpan;
pub use span_ops::{AddSpan, ExpandSpan, SpanUtil};

pub type Spanned<T> = (T, SourceSpan);
