/// Allows to obtain values in associated types.
pub trait Get<T> {
    fn get() -> T;
}
