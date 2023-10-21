use super::*;
use chrono::{Duration, NaiveDateTime};
use korvin_core::{element_builder::ElementBuilder, web_sys::InputEvent};

pub enum TimerMessage {
    SetMode(TimerMode),
    SetDuration(Duration),
}
#[derive(Debug)]
pub struct RunningTimer {
    since: NaiveDateTime,
}

impl std::default::Default for RunningTimer {
    fn default() -> Self {
        Self { since: now() }
    }
}

#[derive(Debug, Default, derive_more::From)]
pub enum TimerMode {
    #[default]
    Stopped,
    Running(RunningTimer),
}

#[derive(Debug)]
pub struct Timer {
    mode: TimerMode,
    duration: Duration,
}

impl std::default::Default for Timer {
    fn default() -> Self {
        Self {
            mode: Default::default(),
            duration: Duration::seconds(20),
        }
    }
}

impl HandleMessage<TimerMessage> for Timer {
    fn handle(&mut self, message: TimerMessage) {
        match message {
            TimerMessage::SetMode(mode) => self.mode = mode,
            TimerMessage::SetDuration(duration) => self.duration = duration,
        }
    }
}

fn now() -> NaiveDateTime {
    chrono::Local::now().naive_local()
}

fn app(communicator: Communicator<TimerMessage>, inner: &Timer) -> ElementBuilder {
    let slider = "span"
        .attribute("class", "slider")
        .child(
            "label"
                .attribute("for", "slider")
                .text(format!("Duration: {}s", inner.duration.num_seconds()).as_str()),
        )
        .child(
            "input"
                .attribute("name", "slider")
                .attribute("id", "slider")
                .attribute("type", "range")
                .attribute("min", 0.to_string().as_str())
                .attribute("max", 60.to_string().as_str())
                .input_value(inner.duration.num_seconds().to_string().as_str())
                .event((), "input", move |input: InputEvent| {
                    input.on_value(communicator, |new: i64| {
                        TimerMessage::SetDuration(Duration::seconds(new))
                    })
                }),
        );
    let progress = |start: NaiveDateTime| {
        let percent = ((now() - start).num_milliseconds() as f64
            / inner.duration.num_milliseconds() as f64)
            .min(1.0)
            * 100.;
        "div"
            .attribute(
                "style",
                "height: 2rem; width: 16rem; background-color: lightblue",
            )
            .child("div".attribute(
                "style",
                format!("height: 100%; width: {percent}%; background-color: blue;",).as_str(),
            ))
    };
    let container = "div".attribute("class", "timer-container").child(slider);
    let start_timer = || TimerMessage::SetMode(TimerMode::Running(RunningTimer { since: now() }));
    let body = match inner.mode {
        TimerMode::Stopped => container
            .child("div".text("stopped"))
            .child(button((), communicator, start_timer).text("start")),
        TimerMode::Running(RunningTimer { since }) => container
            .child(progress(since))
            .child("div".text(format!("running: {}", (now() - since).min(inner.duration)).as_str()))
            .child(button((), communicator, start_timer).text("reset")),
    };
    "main"
        .attribute("class", "flight-booker")
        .child("h3".text("7 GUIs: Timer"))
        .child(body)
}

impl ToLazyHtml for WithCommunicator<Timer, TimerMessage> {
    fn to_lazy_html(&self) -> ElementBuilder {
        let Self {
            inner,
            communicator,
        } = self;
        app(*communicator, inner)
    }
}

/// example of a custom runtime
pub fn run() -> Result<()> {
    let body = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.body())
        .ok_or_else(|| eyre!("no body"))?;

    let (rx, mut with_communicator) = example_7_guis::example_4_timer::Timer::default()
        .with_communicator::<example_7_guis::example_4_timer::TimerMessage>(
    );
    wasm_bindgen_futures::spawn_local(async move {
        let mut runtime = Runtime::new(body);

        if let Err(message) = runtime
            .dom_executor
            .rebuild(with_communicator.to_lazy_html().build())
        {
            tracing::error!(?message, "rebuilding failed");
        }

        enum AppEvent {
            Message(example_7_guis::example_4_timer::TimerMessage),
            Tick,
        }
        let rx = utils::stream_compat::UnboundedReceiverStream::new(rx)
            .map(AppEvent::Message)
            .boxed_local();
        let mut app_events =
            futures::stream::iter([rx, interval(1).map(|_| AppEvent::Tick).boxed_local()])
                .flatten_unordered(2);
        while let Some(event) = app_events.next().await {
            match event {
                AppEvent::Message(message) => {
                    with_communicator.inner.handle(message);
                    if let Err(message) = runtime
                        .dom_executor
                        .rebuild(with_communicator.to_lazy_html().build())
                    {
                        tracing::error!(?message, "rebuilding failed");
                    }
                }
                AppEvent::Tick => {
                    if let Err(message) = runtime
                        .dom_executor
                        .rebuild(with_communicator.to_lazy_html().build())
                    {
                        tracing::error!(?message, "rebuilding failed");
                    }
                }
            }
        }
    });
    Ok(())
}
