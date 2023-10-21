use std::sync::Arc;

use js_sys::Function;
pub use wasm_bindgen::closure::IntoWasmClosure;
use wasm_bindgen::{prelude::Closure, JsCast};

#[derive(PartialEq, Debug, Clone, derive_more::Display)]
pub struct EventName(Value);

impl AsRef<str> for EventName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

pub type WebSysClosure<Args> = Closure<dyn FnMut(Args)>;

use super::Value;

pub struct KorvinClosure<EventKind> {
    pub hash: u64,
    pub closure: Arc<WebSysClosure<EventKind>>,
}

impl<E> std::hash::Hash for KorvinClosure<E> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state)
    }
}

impl<E> std::clone::Clone for KorvinClosure<E> {
    fn clone(&self) -> Self {
        let Self { hash, closure } = self;
        Self {
            hash: *hash,
            closure: Arc::clone(closure),
        }
    }
}

impl<E> KorvinClosure<E> {
    pub fn js_function(&self) -> &Function {
        self.closure.as_ref().as_ref().unchecked_ref()
    }
}
impl<Closure> PartialEq for KorvinClosure<Closure> {
    fn eq(&self, other: &Self) -> bool {
        self.hash.eq(&other.hash)
    }
}

impl<Closure> std::fmt::Debug for KorvinClosure<Closure> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("hash", &self.hash)
            .finish_non_exhaustive()
    }
}

impl<T> From<T> for EventName
where
    T: Into<Value>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
