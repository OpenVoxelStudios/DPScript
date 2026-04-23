pub trait Indent {
    fn indent(self, amount: usize) -> String;
}

impl Indent for String {
    fn indent(self, amount: usize) -> String {
        self.split("\n")
            .map(|it| format!("{}{it}", " ".repeat(amount)))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
