use super::*;
use chrono::NaiveDate;
use korvin_core::{element_builder::ElementBuilder, web_sys::MouseEvent};

pub enum FlightBookerMessage {
    SetMode(FlightBookerMode),
}

#[derive(Debug)]
pub enum FlightBookerMode {
    OneWayFlight {
        start: chrono::NaiveDate,
    },
    ReturnFlight {
        start: chrono::NaiveDate,
        end: chrono::NaiveDate,
    },
}

impl std::default::Default for FlightBookerMode {
    fn default() -> Self {
        Self::OneWayFlight { start: today() }
    }
}

#[derive(Default, Debug)]
pub struct FlightBooker {
    mode: FlightBookerMode,
}

impl HandleMessage<FlightBookerMessage> for FlightBooker {
    fn handle(&mut self, message: FlightBookerMessage) {
        match message {
            FlightBookerMessage::SetMode(mode) => self.mode = mode,
        }
    }
}

fn today() -> NaiveDate {
    chrono::Local::now().date_naive()
}

fn app(communicator: Communicator<FlightBookerMessage>, inner: &FlightBooker) -> ElementBuilder {
    let body = {
        let one_way_flight = "One way flight";
        let return_flight = "Return flight";
        let start = match inner.mode {
            FlightBookerMode::OneWayFlight { start } => start,
            FlightBookerMode::ReturnFlight { start, end: _ } => start,
        };
        let end = match inner.mode {
            FlightBookerMode::OneWayFlight { start: _ } => None,
            FlightBookerMode::ReturnFlight { start: _, end } => Some(end),
        };
        let select = "select"
            .attribute(
                "value",
                match inner.mode {
                    FlightBookerMode::OneWayFlight { .. } => one_way_flight,
                    FlightBookerMode::ReturnFlight { .. } => return_flight,
                },
            )
            .child(
                "option"
                    .attribute("value", one_way_flight)
                    .text(one_way_flight)
                    .event((start, end), "mousedown", move |_: MouseEvent| {
                        communicator.send(FlightBookerMessage::SetMode(
                            FlightBookerMode::OneWayFlight { start },
                        ))
                    }),
            )
            .child(
                "option"
                    .attribute("value", return_flight)
                    .text(return_flight)
                    .event((start, end), "click", move |_: MouseEvent| {
                        communicator.send(FlightBookerMessage::SetMode(
                            FlightBookerMode::ReturnFlight {
                                start,
                                end: end.unwrap_or_else(today),
                            },
                        ))
                    }),
            );
        let inputs = {
            let container = "div";
            match inner.mode {
                FlightBookerMode::OneWayFlight { start } => {
                    container.child(input((), communicator, start, |start| {
                        FlightBookerMessage::SetMode(FlightBookerMode::OneWayFlight { start })
                    }))
                }
                FlightBookerMode::ReturnFlight { start, end } => container
                    .child(input(end, communicator, start, move |start| {
                        FlightBookerMessage::SetMode(FlightBookerMode::ReturnFlight { start, end })
                    }))
                    .child(input(start, communicator, end, move |end| {
                        FlightBookerMessage::SetMode(FlightBookerMode::ReturnFlight { start, end })
                    })),
            }
        };
        "div".child(select).child(inputs)
    };
    "main"
        .attribute("class", "flight-booker")
        .child("h3".text("7 GUIs: Flight Booker"))
        .child("div".child(body))
        .child("div".text(format!("{inner:#?}").as_str()))
}

impl ToLazyHtml for WithCommunicator<FlightBooker, FlightBookerMessage> {
    fn to_lazy_html(&self) -> ElementBuilder {
        let Self {
            inner,
            communicator,
        } = self;
        app(*communicator, inner)
    }
}
