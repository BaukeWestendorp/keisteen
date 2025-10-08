pub trait Predicate<T> {
    fn test(&self, t: &T) -> bool;
}
