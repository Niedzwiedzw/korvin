use crate::data::{AttributeName, AttributeValue, TagName};
use std::any::TypeId;
use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Debug, Error)]
#[error("JsError: {message}")]
pub struct JsError {
    message: String,
}

impl From<JsValue> for JsError {
    fn from(value: JsValue) -> Self {
        Self {
            message: format!("{value:#?}"),
        }
    }
}

pub struct DebugOf {
    _ty: TypeId,
    debug: String,
}

impl DebugOf {
    pub fn new<T: std::fmt::Debug + 'static>(val: &T) -> Self {
        Self {
            _ty: std::any::TypeId::of::<T>(),
            debug: format!("{val:?}"),
        }
    }
}

impl std::fmt::Debug for DebugOf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.debug)
    }
}

#[derive(Debug, Error)]
pub enum RawOperationError {
    #[error("Inserting children into parent {to_parent:?}. ({source})")]
    InsertingChildren { to_parent: DebugOf, source: JsError },
    #[error("Picking up children from_parent {from_parent:?}. ({source})")]
    PickingUpChildren {
        from_parent: DebugOf,
        source: JsError,
    },
    #[error("Reparenting: from_parent {from_parent:?},  to_parent: {to_parent:?}. ({source})")]
    Reparenting {
        from_parent: DebugOf,
        to_parent: DebugOf,
        source: JsError,
    },
    #[error("Creation of element failed: {kind:?}: {source}")]
    CreatingElement { kind: TagName, source: JsError },
    #[error("Insertion of element failed: {element:?} -> {to:?}. ({source})")]
    InsertElement {
        to: DebugOf,
        element: DebugOf,
        source: JsError,
    },
    #[error("Revoval of element {element:?} from {from_parent:?}. ({source})")]
    RemoveElement {
        from_parent: DebugOf,
        element: DebugOf,
        source: JsError,
    },
    #[error("Setting attribute on element {element:?}: ({attribute} -> {value:?}): {source}")]
    SetAttribute {
        element: DebugOf,
        attribute: AttributeName,
        value: Option<AttributeValue>,
        source: JsError,
    },
    #[error("Unhandled js error: {0}")]
    UnhandledJs(#[from] JsError),
    #[error("Element mismatch found on removal, this is a fatal error and should be reported to korvin developers. ({expected:?} != {got:?})")]
    ElementMismatch { expected: DebugOf, got: DebugOf },
    #[error("Adding event listener: {0}")]
    AddEventListener(#[source] JsError),
    #[error("Removing event listener: {0}")]
    RemoveEventListener(#[source] JsError),
    #[error("Replacing an {element:?} with {with:?}")]
    SwappingElements {
        element: DebugOf,
        with: DebugOf,
        source: JsError,
    },
    #[error("Expected {element:?} to be an <input> element.")]
    NotAnInputElement { element: DebugOf },
}

pub type RawOperationResult<T> = std::result::Result<T, RawOperationError>;
