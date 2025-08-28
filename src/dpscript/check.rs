pub trait CheckConst {
    fn is_const(&self) -> bool;
}

impl<T: CheckConst> CheckConst for Vec<T> {
    fn is_const(&self) -> bool {
        self.iter().all(T::is_const)
    }
}
