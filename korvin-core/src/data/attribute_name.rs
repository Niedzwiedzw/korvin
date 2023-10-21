use super::Value;

#[derive(PartialEq, Debug, Clone, derive_more::Display, PartialOrd, Ord, Eq, Hash)]
pub struct AttributeName(Value);

impl<T> From<T> for AttributeName
where
    T: Into<Value>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl AsRef<str> for AttributeName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
