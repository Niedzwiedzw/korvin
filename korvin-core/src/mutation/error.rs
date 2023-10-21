use thiserror::Error;

use crate::{document_model::error::DocumentModelError, raw_operations::error::RawOperationError};
#[derive(Debug, Error)]
pub enum MutationError {
    #[error("Creating element: {0}")]
    ElementCreate(#[source] RawOperationError),
    #[error("Uncreating element: {0}")]
    ElementUncreate(#[source] RawOperationError),
    #[error("Inserting element: {0}")]
    ElementInsert(#[source] RawOperationError),
    #[error("Uninserting element: {0}")]
    ElementUninsert(#[source] RawOperationError),
    #[error("Setting attribute: {0}")]
    SetAttribute(#[source] RawOperationError),
    #[error("Setting attribute: {0}")]
    UnsetAttribute(#[source] RawOperationError),
    #[error("Adding event listener: {0}")]
    ElementAddEventListener(#[source] RawOperationError),
    #[error("Removing event listener: {0}")]
    ElementRemoveEventListener(#[source] RawOperationError),
    #[error("No element is currently being built")]
    NoElementInBuilder,
    #[error("Document model went out of sync with the actual DOM: {0}")]
    DocumentModel(#[source] DocumentModelError),
    #[error("Setting <input> node's .value failed.")]
    SetInputValue(#[source] RawOperationError),
}

pub type MutationResult<T> = std::result::Result<T, MutationError>;
