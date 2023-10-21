use data::ElementId;
use dom_executor::DomExecutor;
pub use js_sys;
use mutation::error::MutationError;
use raw_operations::error::{DebugOf, RawOperationError};
use thiserror::Error;
pub use web_sys;
use web_sys::{Document, Element};
pub mod flavors;

thread_local! {
    pub static DOCUMENT: web_sys::Document = crate::get_document().expect("document not present");
}

pub mod data;
pub mod document_model;
pub mod dom_executor;
pub mod element_builder;
pub mod mutation;
pub mod raw_operations;
pub mod utils;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("No window")]
    NoWindow,
    #[error("No document")]
    NoDocument,
    #[error("No root element (body)")]
    NoRootElement,
    #[error("Executing a mutation: {0}")]
    Mutation(#[source] mutation::error::MutationError),
    #[error("Nothing to revert, revert stack is empty")]
    UndoStackEmpty,
    #[error("Document model didn't clean up properly on previous draw.")]
    ElementStackNonEmpty,
    #[error("DocumentModel modification failed: {0}")]
    DocumentModel(#[source] document_model::error::DocumentModelError),
    #[error(
        "Tried popping unexpected element: got root [{current_root:?}], exepected root: [{expected_root:?}]"
    )]
    InvalidElementPopped {
        current_root: DebugOf,
        expected_root: DebugOf,
    },
    #[error("Slicers went out of sync: {message}")]
    SlicersWentOutOfSync { message: String },
    #[error("Error occurred when reparenting: {source}")]
    Reparenting { source: RawOperationError },
    #[error("When reverting trailing mutations: {0}")]
    UndoingTrailingMutations(#[source] MutationError),
    #[error("When performing new mutations: {0}")]
    PerformingNewMutations(#[source] MutationError),
    #[error("This should never happen, but runtime crashed on previous redraw.")]
    RuntimeCrashedOnPreviousRedraw,
    #[error("Reinserting old child resulted in an error.")]
    ReinsertingOldChild(#[source] RawOperationError),
    #[error("Removing old child.")]
    RemovingElement(#[source] RawOperationError),
}

type RuntimeResult<T> = std::result::Result<T, RuntimeError>;

pub fn get_document() -> RuntimeResult<Document> {
    web_sys::window()
        .ok_or(RuntimeError::NoWindow)
        .and_then(|window| window.document().ok_or(RuntimeError::NoDocument))
}

#[derive(Debug)]
pub struct Runtime {
    pub dom_executor: DomExecutor,
}

impl Runtime {
    pub fn root_element(&self) -> &ElementId {
        self.dom_executor
            .executed
            .as_ref()
            .map(|executed| &executed.element.create.log.element_id)
            .expect("runtime crashed")
    }
    pub fn new(root_element: impl Into<Element>) -> Self {
        Self {
            dom_executor: DomExecutor::new(ElementId::new(root_element.into())),
        }
    }
}
