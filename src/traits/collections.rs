/// A trait defining whether a collection contains an element
pub trait Contains<T: Eq> {
    fn has(&self, item: &T) -> bool;
}

/// Implementation for Vec
impl<T: Eq> Contains<T> for Vec<T> {
    fn has(&self, item: &T) -> bool {
        self.iter().any(|element| element == item)
    }
}
