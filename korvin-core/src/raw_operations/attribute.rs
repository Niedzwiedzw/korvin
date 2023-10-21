pub mod error {
    use thiserror::Error;

    use crate::data::Value;

    #[derive(Debug, Error)]
    pub enum AttributeError {
        #[error("Invalid attribute name: {name}")]
        InvalidAttributeName { name: Value },
    }

    pub type AttributeResult<T> = std::result::Result<T, AttributeError>;
}
