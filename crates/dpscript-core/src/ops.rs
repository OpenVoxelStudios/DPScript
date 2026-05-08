pub trait MinBy {
    type Item;

    fn min_by<I: PartialOrd, F: Fn(&Self::Item) -> I>(self, f: F) -> Self::Item;
}

impl<T> MinBy for (T, T) {
    type Item = T;

    fn min_by<I: PartialOrd, F: Fn(&Self::Item) -> I>(self, f: F) -> Self::Item {
        let a = f(&self.0);
        let b = f(&self.1);

        if a < b { self.0 } else { self.1 }
    }
}

pub trait MaxBy {
    type Item;

    fn max_by<I: PartialOrd, F: Fn(&Self::Item) -> I>(self, f: F) -> Self::Item;
}

impl<T> MaxBy for (T, T) {
    type Item = T;

    fn max_by<I: PartialOrd, F: Fn(&Self::Item) -> I>(self, f: F) -> Self::Item {
        let a = f(&self.0);
        let b = f(&self.1);

        if a > b { self.0 } else { self.1 }
    }
}
