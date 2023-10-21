use korvin_core::{element_builder::ElementBuilder, Runtime};
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);
use eyre::{eyre, Result, WrapErr};
use korvin_core::element_builder::AsElementBuilder;
use web_sys::Element;

use crate::utils::Timeout;
pub mod utils {
    use js_sys::Promise;
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use std::time::Duration;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;

    pub struct Timeout {
        id: JsValue,
        inner: JsFuture,
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = setTimeout)]
        fn set_timeout(closure: JsValue, millis: f64) -> JsValue;

        #[wasm_bindgen(js_name = clearTimeout)]
        fn clear_timeout(id: &JsValue);
    }

    impl Timeout {
        pub fn new(dur: Duration) -> Timeout {
            let millis = dur
                .as_secs()
                .checked_mul(1000)
                .unwrap()
                .checked_add(dur.subsec_millis() as u64)
                .unwrap() as f64; // TODO: checked cast

            let mut id = None;
            let promise = Promise::new(&mut |resolve, _reject| {
                id = Some(set_timeout(resolve.into(), millis));
            });

            Timeout {
                id: id.unwrap(),
                inner: JsFuture::from(promise),
            }
        }
    }

    impl Future for Timeout {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<()> {
            Pin::new(&mut self.inner).poll(cx).map(|_| ())
        }
    }

    impl Drop for Timeout {
        fn drop(&mut self) {
            clear_timeout(&self.id);
        }
    }
}

struct RuntimeTester<'runtime> {
    runtime: &'runtime Runtime,
}

impl<'runtime> RuntimeTester<'runtime> {
    fn root_element(&self) -> &Element {
        self.runtime.root_element().as_ref()
    }

    fn assert_contains_html(&self, html: &str) -> Result<()> {
        let actual = self.root_element().inner_html();
        actual
            .contains(html)
            .then_some(())
            .ok_or_else(|| eyre!("expected following html: {html}\n\nfound:\n{actual}\n\n"))
    }
    fn assert_not_contains_html(&self, html: &str) -> Result<()> {
        self.assert_contains_html(html)
            .is_err()
            .then_some(())
            .ok_or_else(|| {
                eyre!(
                    "unexpected html: {html}\n\nfound in: {}",
                    self.root_element().inner_html()
                )
            })
    }
}

trait RuntimeTestExt {
    fn test(&self) -> RuntimeTester<'_>;
}

impl RuntimeTestExt for Runtime {
    fn test(&self) -> RuntimeTester<'_> {
        RuntimeTester { runtime: self }
    }
}

pub fn window() -> Result<web_sys::Window> {
    web_sys::window().ok_or_else(|| eyre!("no window"))
}

pub fn document() -> Result<web_sys::Document> {
    window().and_then(|window| window.document().ok_or_else(|| eyre!("no document")))
}

pub fn root_element(name: &str) -> Result<web_sys::Element> {
    document().and_then(|document| {
        document
            .create_element(name)
            .map_err(|e| eyre!("{e:#?}"))
            .wrap_err("creating element")
            .and_then(|element| {
                document
                    .body()
                    .ok_or_else(|| eyre!("no body"))
                    .and_then(|body| {
                        body.append_child(&element)
                            .map_err(|e| eyre!("{e:#?}"))
                            .map(|_| element)
                    })
            })
    })
}

pub fn runtime(root_name: &str) -> Result<Runtime> {
    root_element(root_name)
        .map(Runtime::new)
        .wrap_err_with(|| format!("creating runtime for <{root_name}/>"))
}

macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        name.strip_suffix("::f")
            .unwrap()
            .replace("::", "-")
            .replace("{", "-")
            .replace("}", "-")
    }};
}

fn setup_tracing() -> Result<()> {
    use tracing_subscriber::{fmt::format::Pretty, prelude::*};
    use tracing_web::{performance_layer, MakeConsoleWriter};
    let fmt_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_ansi(true) // Only partially supported across browsers
        .without_time()
        .with_writer(MakeConsoleWriter); // write events to the console
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .with(tracing_subscriber::EnvFilter::new("trace"))
        .try_init()?; // Install these as subscribers to tracing events
    Ok(())
}
fn wait_for_logs() -> Timeout {
    Timeout::new(std::time::Duration::from_millis(100))
}

macro_rules! runtime {
    () => {{
        if setup_tracing().is_ok() {
            Timeout::new(std::time::Duration::from_millis(2000)).await;
        }
        let function_name = function!();
        runtime(&function_name).wrap_err_with(|| format!("spawning runtime for {}", function_name))
    }};
}

pub mod empty_app {
    use korvin_core::element_builder::ElementWithChildrenRecipe;

    use super::*;

    pub fn app() -> ElementWithChildrenRecipe {
        ElementBuilder::builder("div").build()
    }
    #[wasm_bindgen_test]
    pub async fn test_app_rebuilds_many_times() -> Result<()> {
        let mut runtime = runtime!()?;
        for i in 0..3 {
            runtime
                .dom_executor
                .rebuild(app())
                .wrap_err_with(|| format!("rebuild no {i}"))?;
        }
        wait_for_logs().await;
        Ok(())
    }
}

pub mod single_element_app {
    use korvin_core::element_builder::ElementWithChildrenRecipe;

    use super::*;

    pub fn app() -> ElementWithChildrenRecipe {
        "single".build()
    }

    #[wasm_bindgen_test]
    pub async fn test_app_rebuilds_many_times() -> Result<()> {
        let mut runtime = runtime!()?;
        for i in 0..3 {
            runtime
                .dom_executor
                .rebuild(app())
                .wrap_err_with(|| format!("rebuild no {i}"))?;
            runtime.test().assert_contains_html("<single>")?;
        }
        wait_for_logs().await;
        Ok(())
    }
}

pub mod nested_app {
    use korvin_core::element_builder::ElementWithChildrenRecipe;

    use super::*;

    pub fn app() -> ElementWithChildrenRecipe {
        "nested".child("child".child("grandchild")).build()
    }

    #[wasm_bindgen_test]
    pub async fn test_app_rebuilds_many_times() -> Result<()> {
        let mut runtime = runtime!()?;
        for i in 0..3 {
            runtime
                .dom_executor
                .rebuild(app())
                .wrap_err_with(|| format!("rebuild no {i}"))?;
            runtime.test().assert_contains_html("<nested>")?;
            runtime.test().assert_contains_html("<child>")?;
            runtime.test().assert_contains_html("<grandchild>")?;
        }
        wait_for_logs().await;
        Ok(())
    }
}

pub mod multiple_children_app {
    use korvin_core::element_builder::ElementWithChildrenRecipe;

    use super::*;

    pub fn app() -> ElementWithChildrenRecipe {
        "nested".child("child").child("sibling").build()
    }

    #[wasm_bindgen_test]
    pub async fn test_app_rebuilds_many_times() -> Result<()> {
        let mut runtime = runtime!()?;
        for i in 0..3 {
            runtime
                .dom_executor
                .rebuild(app())
                .wrap_err_with(|| format!("rebuild no {i}"))?;
            runtime.test().assert_contains_html("<nested>")?;
            runtime.test().assert_contains_html("<child>")?;
            runtime.test().assert_contains_html("<sibling>")?;
        }
        wait_for_logs().await;
        Ok(())
    }
}

pub mod mutable_text_app {

    use super::*;

    #[wasm_bindgen_test]
    pub async fn test_app_rebuilds_many_times() -> Result<()> {
        let child_text = |count: i32| format!("child redraw number {count}");
        let app = |count: i32| "mutable_text_app".text(child_text(count).as_str()).build();

        let mut runtime = runtime!()?;
        for i in 0..3 {
            runtime
                .dom_executor
                .rebuild(app(i))
                .wrap_err_with(|| format!("rebuild no {i}"))?;
            runtime.test().assert_contains_html(&child_text(i))?;
        }
        wait_for_logs().await;
        Ok(())
    }
}

pub mod growing_children {
    use super::*;

    #[wasm_bindgen_test]
    pub async fn test_app_rebuilds_many_times() -> Result<()> {
        let text = |id: i32| format!("child {id}");
        let text_child = |id: i32| "div".text(text(id).as_str());
        let count = |i| (0..i);
        let app = |i: i32| {
            "mutable_text_app"
                .children(count(i).map(text_child))
                .build()
        };

        let mut runtime = runtime!()?;
        for i in 0..3 {
            runtime
                .dom_executor
                .rebuild(app(i))
                .wrap_err_with(|| format!("rebuild no {i}"))?;
            count(i).map(text).try_for_each(|expected_text| {
                runtime.test().assert_contains_html(&expected_text)
            })?;
        }
        wait_for_logs().await;
        Ok(())
    }
}
pub mod shrinking_children {
    use super::*;

    #[wasm_bindgen_test]
    pub async fn test_app_rebuilds_many_times() -> Result<()> {
        let text = |id: i32| format!("child {id}");
        let text_child = |id: i32| "div".text(text(id).as_str());
        let count = |i| (0..i);
        let app = |i: i32| {
            "mutable_text_app"
                .children(count(i).map(text_child))
                .build()
        };

        let mut runtime = runtime!()?;
        for i in (0..5).rev() {
            runtime
                .dom_executor
                .rebuild(app(i))
                .wrap_err_with(|| format!("rebuild no {i}"))?;
            count(i).map(text).try_for_each(|expected_text| {
                runtime.test().assert_contains_html(&expected_text)
            })?;
        }
        wait_for_logs().await;
        Ok(())
    }
}
pub mod shrinking_children_with_leftover_and_attributes {
    use super::*;

    #[wasm_bindgen_test]
    pub async fn test_app_rebuilds_many_times() -> Result<()> {
        let text = |id: i32| format!("child {id}");
        let text_child = |id: i32| {
            "div"
                .attribute("style", "color: red;")
                .text(text(id).as_str())
        };
        let count = |i| (0..i);
        const CONST_TEXT_1: &str = "this should always be here 1";
        const CONST_TEXT_2: &str = "this should always be here 2";
        let app = |i: i32| {
            "mutable_text_app"
                .child("div".text(CONST_TEXT_1))
                .children(count(i).map(text_child))
                .child("div".text(CONST_TEXT_2))
                .build()
        };

        let mut runtime = runtime!()?;
        for i in (0..5).rev() {
            runtime
                .dom_executor
                .rebuild(app(i))
                .wrap_err_with(|| format!("rebuild no {i}"))?;
            count(i)
                .try_for_each(|id| runtime.test().assert_contains_html(&text(id)))
                .and_then(|_| runtime.test().assert_contains_html(CONST_TEXT_1))
                .and_then(|_| runtime.test().assert_contains_html(CONST_TEXT_2))
                .and_then(|_| runtime.test().assert_not_contains_html(&text(i + 1)))?;
        }
        wait_for_logs().await;
        Ok(())
    }
}
pub mod toggling_children_with_leftover_and_attributes {
    use super::*;

    #[wasm_bindgen_test]
    pub async fn test_app_rebuilds_many_times() -> Result<()> {
        let text = |id: i32| format!("child {id}");
        let text_child = |id: i32| {
            "div"
                .attribute("style", "color: red;")
                .text(text(id).as_str())
        };
        let count = |i| (0..i);
        let is_even = |id: i32| (id % 2 == 2);
        const CONST_TEXT_1: &str = "this should always be here 1";
        const EVEN_ONLY_TEXT: &str = "this is only here when even";
        let app = |i: i32| {
            "mutable_text_app"
                .child("div".text(CONST_TEXT_1))
                .children(count(i).map(text_child))
                .children(is_even(i).then_some("div".text(CONST_TEXT_1)))
                .build()
        };

        let mut runtime = runtime!()?;
        for i in (0..5).rev() {
            runtime
                .dom_executor
                .rebuild(app(i))
                .wrap_err_with(|| format!("rebuild no {i}"))?;
            count(i)
                .try_for_each(|id| runtime.test().assert_contains_html(&text(id)))
                .and_then(|_| match is_even(i) {
                    true => runtime.test().assert_contains_html(EVEN_ONLY_TEXT),
                    false => Ok(()),
                })
                .and_then(|_| runtime.test().assert_contains_html(CONST_TEXT_1))
                .and_then(|_| runtime.test().assert_not_contains_html(&text(i + 1)))?;
        }
        wait_for_logs().await;
        Ok(())
    }
}
pub mod optional_sibling {
    use super::*;

    #[derive(Debug)]
    pub struct AppState {
        count: usize,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum AppMessage {
        Increment(usize),
        Decrement(usize),
        SetNewValue(usize),
    }
    impl AppState {
        pub fn handle(&mut self, message: AppMessage) {
            match message {
                AppMessage::Increment(by) => {
                    self.count = self.count.saturating_add(by);
                }
                AppMessage::Decrement(by) => {
                    self.count = self.count.saturating_sub(by);
                }
                AppMessage::SetNewValue(new) => self.count = new,
            }
        }
    }

    #[wasm_bindgen_test]
    pub async fn test_app_rebuilds_many_times() -> Result<()> {
        let mut runtime = runtime!()?;

        let app = |count: usize| {
            let el = |name: &str| ElementBuilder::builder(name);
            el("main")
                .children(
                    ((count % 2) == 0)
                        .then_some(())
                        .into_iter()
                        .map(|_| el("sometimes-here").text("this is even")),
                )
                .child(el("always-here"))
                .build()
        };
        for i in 0..5 {
            tracing::debug!(" ------- REBUILD NUMBER #{i} -------\n\n");
            runtime
                .dom_executor
                .rebuild(app(i))
                .wrap_err_with(|| format!("rebuild no {i}"))?;

            tracing::debug!(" ------- END OF REBUILD NUMBER #{i} -------");
        }
        wait_for_logs().await;
        Ok(())
    }
}
