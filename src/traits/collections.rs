/// A trait defining whether a collection contains an element
pub trait Contains<T: Eq> {
    fn contains(&self, item: &T) -> bool;
}

/// A trait defining whether a collection contains an element based on a predicate
pub trait ContainsPredicate<T> {
    fn contains(&self, predicate: impl Fn(&T) -> bool) -> bool;
}

/// Implementation for Vec
impl<T: Eq> Contains<T> for Vec<T> {
    fn contains(&self, item: &T) -> bool {
        self.iter().any(|element| element == item)
    }
}

impl<T> ContainsPredicate<T> for Vec<T> {
    fn contains(&self, predicate: impl Fn(&T) -> bool) -> bool {
        self.iter().any(predicate)
    }
}
