use super::Value;

#[derive(PartialEq, Debug, Clone, derive_more::Display, Eq, Hash, PartialOrd)]
pub struct AttributeValue(Value);

impl AsRef<str> for AttributeValue {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<T> From<T> for AttributeValue
where
    T: Into<Value>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
