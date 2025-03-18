pub trait Contains<T: Eq> {
    fn has(&self, item: T) -> bool;
}

impl<T: Eq> Contains<T> for Vec<T> {
    fn has(&self, item: T) -> bool {
        self.iter().find(|element| **element == item).is_some()
    }
}
