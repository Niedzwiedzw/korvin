pub trait BoxedIterator<T> {
    fn boxed(self) -> Box<dyn Iterator<Item = T>>;
}

impl<I, T> BoxedIterator<T> for I
where
    I: Iterator<Item = T> + 'static,
{
    fn boxed(self) -> Box<dyn Iterator<Item = T>> {
        Box::new(self) as _
    }
}
