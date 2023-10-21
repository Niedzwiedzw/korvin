use eyre::{eyre, Result};
use futures_util::{Stream, StreamExt};
use korvin_core::{
    element_builder::AsElementBuilder,
    web_sys::{self},
    Runtime,
};
use tracing_subscriber::fmt::format::Pretty;
use utils::{button, input, Communicator, InputEventExt, WithCommunicatorExt};
use utils::{HandleMessage, ToLazyHtml, WithCommunicator, WithEventHandling};

pub mod utils;

fn setup_logging() -> Result<()> {
    use tracing_subscriber::prelude::*;
    use tracing_web::{performance_layer, MakeConsoleWriter};
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let fmt_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_ansi(false) // Only partially supported across browsers
        .without_time()
        // .with_timer(UtcTime::rfc_3339()) // see also note below
        .with_writer(MakeConsoleWriter); // write events to the console
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());
    let target = {
        #[cfg(debug_assertions)]
        {
            "trace"
        }
        #[cfg(not(debug_assertions))]
        {
            "info,korvin-core=warn"
        }
    };
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .with(
            tracing_subscriber::EnvFilter::try_new(target)
                .map_err(|e| format!("{e:?}"))
                .unwrap_or_default(),
        )
        .init(); // Install these as subscribers to tracing events
    tracing::info!("logging works");
    Ok(())
}

impl<T, M> WithEventHandling<T, M> {
    fn run(self) -> Result<()>
    where
        M: 'static,
        T: HandleMessage<M> + 'static,
        WithCommunicator<T, M>: ToLazyHtml,
    {
        let body = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.body())
            .ok_or_else(|| eyre!("no body"))?;
        wasm_bindgen_futures::spawn_local(async move {
            let mut runtime = Runtime::new(body);

            let Self {
                mut with_communicator,
                mut rx,
            } = self;

            if let Err(message) = runtime
                .dom_executor
                .rebuild(with_communicator.to_lazy_html().build())
            {
                tracing::error!(?message, "rebuilding failed");
            }
            while let Some(message) = rx.next().await {
                with_communicator.inner.handle(message);
                if let Err(message) = runtime
                    .dom_executor
                    .rebuild(with_communicator.to_lazy_html().build())
                {
                    tracing::error!(?message, "rebuilding failed");
                }
            }
        });
        Ok(())
    }
}

pub mod example_7_guis;

pub async fn sleep_ms(ms: i32) {
    let promise = korvin_core::js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms)
            .unwrap();
    });
    wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .expect("sleep failed");
}

fn interval(interval: i32) -> impl Stream<Item = ()> {
    futures::stream::repeat(()).then(move |()| sleep_ms(interval))
}

pub fn app() -> Result<()> {
    setup_logging()?;
    example_7_guis::example_1_counter::Counter::default()
        .with_example_event_handling()
        .run()?;
    example_7_guis::example_2_temperature_converter::TemperatureConverter::default()
        .with_example_event_handling()
        .run()?;
    example_7_guis::example_3_flight_booker::FlightBooker::default()
        .with_example_event_handling()
        .run()?;
    // custom runtime example
    example_7_guis::example_4_timer::run()?;
    example_7_guis::example_5_crud::Crud::default()
        .with_example_event_handling()
        .run()?;

    Ok(())
}

fn main() {
    tracing::info!("Wasm loaded");
    if let Err(message) = app() {
        tracing::error!(?message, "app crashed");
    }
}
