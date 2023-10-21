use super::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TagName(Value);

impl AsRef<str> for TagName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<T> From<T> for TagName
where
    T: Into<Value>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
