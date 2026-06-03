use miette::{MietteSpanContents, SourceCode};

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Facet, Default,
)]
pub struct NamedSource<'a> {
    pub file: &'a str,
    pub code: &'a str,
}

impl<'a> Into<miette::NamedSource<String>> for NamedSource<'a> {
    fn into(self) -> miette::NamedSource<String> {
        miette::NamedSource::new(self.file, self.code.into())
    }
}

impl<'a> SourceCode for NamedSource<'a> {
    fn read_span<'b>(
        &'b self,
        span: &miette::SourceSpan,
        context_lines_before: usize,
        context_lines_after: usize,
    ) -> Result<Box<dyn miette::SpanContents<'b> + 'b>, miette::MietteError> {
        let inner_contents =
            self.code
                .read_span(span, context_lines_before, context_lines_after)?;

        let contents = MietteSpanContents::new_named(
            self.file.into(),
            inner_contents.data(),
            *inner_contents.span(),
            inner_contents.line(),
            inner_contents.column(),
            inner_contents.line_count(),
        );

        Ok(Box::new(contents))
    }
}
