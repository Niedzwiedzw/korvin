use crate::utils::InputEventExt;

use super::*;
use korvin_core::{element_builder::ElementBuilder, web_sys::InputEvent};

impl HandleMessage<TemperatureConverterMessage> for TemperatureConverter {
    fn handle(&mut self, message: TemperatureConverterMessage) {
        handle_message(self, message)
    }
}

pub enum TemperatureConverterMessage {
    FahrenheitChanged(f32),
    CelciusChanged(f32),
}

#[derive(Default)]
pub struct TemperatureConverter {
    fahrenheit: f32,
    celcius: f32,
}

fn handle_message(state: &mut TemperatureConverter, message: TemperatureConverterMessage) {
    match message {
        TemperatureConverterMessage::FahrenheitChanged(fahrenheit) => {
            state.fahrenheit = fahrenheit;
            state.celcius = (fahrenheit - 32.) * (5. / 9.)
        }
        TemperatureConverterMessage::CelciusChanged(celcius) => {
            state.celcius = celcius;
            state.fahrenheit = celcius * (9. / 5.) + 32.;
        }
    }
}

fn labeled_input(value: f32, label: &str) -> ElementBuilder {
    "span"
        .attribute("class", label)
        .child("input".input_value(value.to_string().as_str()))
        .text(label)
        .key(label)
}

fn app(
    communicator: Communicator<TemperatureConverterMessage>,
    inner: &TemperatureConverter,
) -> ElementBuilder {
    let on_fahrenheit_changed = move |input: InputEvent| match input.value() {
        Ok(v) => communicator.send(TemperatureConverterMessage::FahrenheitChanged(v)),
        Err(message) => {
            tracing::error!(?message, "bad input");
        }
    };
    let on_celcius_changed = move |input: InputEvent| match input.value() {
        Ok(v) => communicator.send(TemperatureConverterMessage::CelciusChanged(v)),
        Err(message) => {
            tracing::error!(?message, "bad input");
        }
    };
    "main"
        .attribute("class", "temperature-converter")
        .child("h3".text("7 GUIs: Temperature Converter"))
        .child(labeled_input(inner.fahrenheit, "fahrenheit").event(
            (),
            "input",
            on_fahrenheit_changed,
        ))
        .child("span".text(" = "))
        .child(labeled_input(inner.celcius, "celcius").event((), "input", on_celcius_changed))
}

impl ToLazyHtml for WithCommunicator<TemperatureConverter, TemperatureConverterMessage> {
    fn to_lazy_html(&self) -> ElementBuilder {
        let Self {
            inner,
            communicator,
        } = self;
        app(*communicator, inner)
    }
}
