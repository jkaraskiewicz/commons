/// A trait defining a conversion from any type to a Result
pub trait ToResult<E> {
    type Source;
    fn to_result(&self, error_fn: impl FnOnce() -> E) -> Result<Self::Source, E>;
}

/// Implementation for bool
/// Returns Err when input is false, and Ok otherwise
impl<E> ToResult<E> for bool {
    type Source = bool;

    fn to_result(&self, error_fn: impl FnOnce() -> E) -> Result<Self::Source, E> {
        if *self {
            Ok(*self)
        } else {
            Err(error_fn())
        }
    }
}
