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

pub trait Body {
    fn body(self) -> String;
}

impl Body for String {
    fn body(self) -> String {
        if self.trim().is_empty() {
            return String::new();
        }

        self.split("\n")
            .map(|it| {
                if it.ends_with(';') || it.trim().is_empty() || it.ends_with(&['(', '{', '[', '<'])
                {
                    it.into()
                } else {
                    format!("{};", it)
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
