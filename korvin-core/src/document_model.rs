use nonempty::NonEmpty;
use tracing::instrument;
use web_sys::Document;

use crate::{data::ElementId, get_document, RuntimeResult};
pub mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum DocumentModelError {
        #[error(
        "Critical error: runtime created an invalid mutation that tried popping the entry point."
    )]
        RootElementPopped,
    }
    pub type DocumentModelResult<T> = std::result::Result<T, DocumentModelError>;
}

#[derive(Clone)]
pub struct DocumentModel {
    pub document: Document,
    pub element_stack: nonempty::NonEmpty<ElementId>,
}

impl std::fmt::Debug for DocumentModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.element_stack
            .iter()
            .try_for_each(|element| write!(f, " -> {element:?}"))
    }
}

impl DocumentModel {
    #[instrument(ret, level = "debug")]
    pub fn push_current_root(&mut self, element: ElementId) {
        self.element_stack.push(element);
    }

    #[instrument(ret, level = "debug")]
    pub fn pop_current_root(&mut self) -> error::DocumentModelResult<ElementId> {
        self.element_stack
            .pop()
            .ok_or(error::DocumentModelError::RootElementPopped)
    }

    pub fn current_root(&self) -> &ElementId {
        self.element_stack.last()
    }

    pub fn new(root: ElementId) -> RuntimeResult<Self> {
        get_document().map(|document| Self {
            document,
            element_stack: NonEmpty::new(root),
        })
    }
}
