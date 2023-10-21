use eyre::{eyre, ContextCompat, Result, WrapErr};
use futures::channel::mpsc::UnboundedReceiver;
pub use korvin_core::flavors::elm_like::Communicator;
use korvin_core::{
    element_builder::AsElementBuilder,
    element_builder::ElementBuilder,
    web_sys::{self, HtmlInputElement, InputEvent, MouseEvent},
};
use std::str::FromStr;
use wasm_bindgen::JsCast;

pub mod stream_compat {
    use futures::channel::mpsc::UnboundedReceiver;
    use futures_util::{Stream, StreamExt};

    use std::pin::Pin;
    use std::task::{Context, Poll};

    #[derive(Debug)]
    pub struct UnboundedReceiverStream<T> {
        inner: UnboundedReceiver<T>,
    }

    impl<T> UnboundedReceiverStream<T> {
        pub fn new(recv: UnboundedReceiver<T>) -> Self {
            Self { inner: recv }
        }

        pub fn into_inner(self) -> UnboundedReceiver<T> {
            self.inner
        }

        pub fn close(&mut self) {
            self.inner.close()
        }
    }

    impl<T> Stream for UnboundedReceiverStream<T> {
        type Item = T;

        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            self.inner.poll_next_unpin(cx)
        }
    }
}

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

pub trait ToLazyHtml {
    fn to_lazy_html(&self) -> ElementBuilder;
}

pub struct WithCommunicator<T, M: 'static> {
    pub inner: T,
    pub communicator: Communicator<M>,
}

pub struct WithEventHandling<T, M: 'static> {
    pub with_communicator: WithCommunicator<T, M>,
    pub rx: UnboundedReceiver<M>,
}

pub trait WithCommunicatorExt: Sized {
    fn with_communicator<M>(self) -> (UnboundedReceiver<M>, WithCommunicator<Self, M>);
    fn with_example_event_handling<M>(self) -> WithEventHandling<Self, M>;
}

impl<T: Sized> WithCommunicatorExt for T {
    fn with_communicator<M>(self) -> (UnboundedReceiver<M>, WithCommunicator<Self, M>) {
        let (rx, communicator) = Communicator::create();
        (
            rx,
            WithCommunicator {
                inner: self,
                communicator,
            },
        )
    }

    fn with_example_event_handling<M>(self) -> WithEventHandling<Self, M> {
        let (rx, with_communicator) = self.with_communicator();
        WithEventHandling {
            with_communicator,
            rx,
        }
    }
}

pub trait HandleMessage<M> {
    fn handle(&mut self, message: M);
}
