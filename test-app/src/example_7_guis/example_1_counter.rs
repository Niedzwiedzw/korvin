use super::*;
use korvin_core::{element_builder::ElementBuilder, web_sys::MouseEvent};

pub enum CounterMessage {
    Increment,
    SetCount(i32),
}
#[derive(Default)]
pub struct Counter {
    count: i32,
}

impl HandleMessage<CounterMessage> for Counter {
    fn handle(&mut self, message: CounterMessage) {
        match message {
            CounterMessage::Increment => self.count += 1,
            CounterMessage::SetCount(count) => self.count = count,
        }
    }
}

impl ToLazyHtml for WithCommunicator<Counter, CounterMessage> {
    fn to_lazy_html(&self) -> ElementBuilder {
        let Self {
            inner,
            communicator,
        } = self;
        let communicator = *communicator;
        "main"
            .child("h3".text("7 GUIs: Counter"))
            .attribute("class", "counter")
            .child(input(
                (),
                communicator,
                inner.count,
                CounterMessage::SetCount,
            ))
            .child(
                "button"
                    .text("Count")
                    .event((), "mousedown", move |_: MouseEvent| {
                        communicator.send(CounterMessage::Increment)
                    }),
            )
    }
}
