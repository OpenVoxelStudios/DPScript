use miette::SourceSpan;

pub type Spanned<T> = (T, SourceSpan);

pub trait AddSpan {
    fn add(&self, other: SourceSpan) -> SourceSpan;
}

impl AddSpan for SourceSpan {
    fn add(&self, other: SourceSpan) -> SourceSpan {
        (
            self.offset(),
            other.offset().saturating_sub(self.offset()) + other.len(),
        )
            .into()
    }
}

pub trait ExpandSpan {
    fn expand(&self, num: usize) -> SourceSpan;
}

impl ExpandSpan for SourceSpan {
    fn expand(&self, num: usize) -> SourceSpan {
        (self.offset(), self.len() + num).into()
    }
}
