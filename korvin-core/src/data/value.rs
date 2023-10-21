use std::sync::Arc;

type ValueInner = Arc<str>;
#[derive(derive_more::Display, Clone, PartialEq, Debug, PartialOrd, Ord, Eq, Hash)]
pub struct Value(ValueInner);

impl<T> From<T> for Value
where
    T: Into<ValueInner>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl AsRef<str> for Value {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
