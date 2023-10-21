use crate::{
    element_builder::{AsElementBuilder, ElementBuilder},
    web_sys::{self, HtmlInputElement, InputEvent, MouseEvent},
};
use eyre::{eyre, ContextCompat, Result, WrapErr};
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use std::{cell::RefCell, str::FromStr};
use wasm_bindgen::JsCast;

pub mod stream_compat;

pub trait InputEventExt {
    fn value<T: FromStr>(&self) -> Result<T>
    where
        <T as FromStr>::Err: std::fmt::Debug;
    fn on_value<M, T: FromStr, F: Fn(T) -> M>(self, communicator: Communicator<M>, handle: F)
    where
        <T as FromStr>::Err: std::fmt::Debug,
        T: FromStr,
        F: Fn(T) -> M;
}

impl InputEventExt for web_sys::InputEvent {
    fn value<T: FromStr>(&self) -> Result<T>
    where
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.target().context("no target").and_then(|t| {
            t.dyn_ref::<HtmlInputElement>()
                .context("not an input element")
                .and_then(|v| {
                    v.value()
                        .parse()
                        .map_err(|e| eyre!("{e:?}"))
                        .wrap_err("parsing value")
                })
        })
    }

    fn on_value<M, T, F>(self, communicator: Communicator<M>, handle: F)
    where
        <T as FromStr>::Err: std::fmt::Debug,
        T: FromStr,
        F: Fn(T) -> M,
    {
        if let Err(message) = self.value().map(|value| communicator.send(handle(value))) {
            tracing::error!(?message, "bad input value");
        }
    }
}

pub fn input<T, M, F>(
    key: impl std::hash::Hash,
    communicator: Communicator<M>,
    value: T,
    callback: F,
) -> ElementBuilder
where
    T: FromStr + std::fmt::Display + 'static,
    <T as FromStr>::Err: std::fmt::Debug,
    F: Fn(T) -> M + 'static + Clone,
{
    "input"
        .input_value(value.to_string().as_str())
        .event(key, "input", move |event: InputEvent| {
            event.on_value(communicator, callback.clone())
        })
}

pub fn button<M, F>(
    key: impl std::hash::Hash,
    communicator: Communicator<M>,
    callback: F,
) -> ElementBuilder
where
    F: (Fn() -> M) + 'static + Clone,
{
    "button".event(key, "mousedown", move |_: MouseEvent| {
        communicator.send(callback())
    })
}

pub struct Communicator<M, N = M>
where
    M: 'static,
{
    map: fn(N) -> M,
    tx: &'static RefCell<UnboundedSender<M>>,
}

impl<M, N> Clone for Communicator<M, N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M, N> Copy for Communicator<M, N> {}

impl<M> Communicator<M> {
    pub fn create() -> (UnboundedReceiver<M>, Self) {
        let (tx, rx) = futures::channel::mpsc::unbounded::<M>();
        (
            rx,
            Self {
                map: std::convert::identity,
                tx: Box::leak(Box::new(RefCell::new(tx))),
            },
        )
    }
}

impl<M, N> Communicator<M, N> {
    pub fn map<O>(self, map: impl Into<fn(O) -> M>) -> Communicator<M, O> {
        Communicator {
            map: map.into(),
            tx: self.tx,
        }
    }
    pub fn send(self, message: N) {
        self.tx
            .borrow_mut()
            .unbounded_send((self.map)(message))
            .expect("runtime crashed, couldn't send a message")
    }
}
