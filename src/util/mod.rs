mod bits;
mod cursor;
mod ident;
mod map;
mod span;
mod versions;

pub use bits::*;
pub use cursor::*;
pub use ident::*;
pub use map::*;
pub use span::*;
pub use versions::*;

use miette::{SourceOffset, SourceSpan};

pub fn fake_span() -> SourceSpan {
    SourceSpan::new(SourceOffset::from_location("", 0, 0), 1)
}
