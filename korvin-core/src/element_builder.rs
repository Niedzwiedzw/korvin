use self::value_cache::IntoJsValue;
use crate::{
    data::{AttributeName, AttributeValue, EventListenerWrapper, KorvinClosure, TagName},
    mutation::{
        element::builder_mutation::{
            marker::create::ElementCreateMutation,
            marker::finish::ElementFinishMutation,
            modify::set_text::ElementSetTextMutation,
            modify::{
                add_event_listener::{by_event_kind, ElementAddEventListenerMutation},
                ElementBuilderModifyMutation,
            },
            modify::{
                set_attribute::ElementSetAttributeMutation,
                set_input_value::ElementSetInputValueMutation,
            },
        },
        traits::Perform,
    },
};
use std::{collections::BTreeMap, hash::Hasher, iter::empty, sync::Arc};
use wasm_bindgen::{
    convert::{FromWasmAbi, RefFromWasmAbi},
    prelude::Closure,
};

pub mod value_cache {
    use super::calculate_hash;
    use crate::data::Value;
    use std::{cell::RefCell, collections::BTreeMap};

    pub trait IntoJsValue: std::hash::Hash + Copy {
        fn into_value(self) -> Value;
    }

    impl<T> IntoJsValue for T
    where
        T: Into<Value> + std::hash::Hash + Copy,
    {
        fn into_value(self) -> Value {
            self.into()
        }
    }

    #[derive(Default)]
    pub(crate) struct ValueCache {
        previous: BTreeMap<u64, Value>,
        current: BTreeMap<u64, Value>,
    }

    impl ValueCache {
        pub(super) fn cached(&mut self, value: impl IntoJsValue) -> Value {
            let hash = calculate_hash(&value);
            self.current
                .entry(hash)
                .or_insert_with(|| {
                    self.previous
                        .remove(&hash)
                        .unwrap_or_else(|| value.into_value())
                })
                .clone()
        }
        pub(crate) fn next_rebuild(&mut self) {
            std::mem::swap(&mut self.current, &mut self.previous);
            self.current.clear();
        }
    }

    thread_local! {
        pub(crate) static VALUE_CACHE: RefCell<ValueCache> = Default::default();
    }
}

pub trait AsElementBuilder {
    fn into_builder(self) -> ElementBuilder;
    fn builder(kind: impl IntoJsValue) -> ElementBuilder {
        ElementBuilder::builder(kind)
    }
    fn text(self, text: impl IntoJsValue) -> ElementBuilder;
    fn input_value(self, value: impl IntoJsValue) -> ElementBuilder;
    fn event<Key: std::hash::Hash, EventKind>(
        self,
        key: Key,
        name: impl IntoJsValue,
        callback: impl Fn(EventKind) + 'static,
    ) -> ElementBuilder
    where
        EventKind: std::fmt::Debug + Sized + RefFromWasmAbi + FromWasmAbi + 'static,
        by_event_kind::ElementAddEventListenerMutation<EventKind>:
            Into<ElementAddEventListenerMutation> + Perform;
    fn child(self, child: impl Into<ElementBuilder>) -> ElementBuilder;
    fn key(self, key: impl std::hash::Hash) -> ElementBuilder;
    fn children(
        self,
        children: impl IntoIterator<Item = impl Into<ElementBuilder>>,
    ) -> ElementBuilder;
    fn attribute(self, attribute: impl IntoJsValue, value: impl IntoJsValue) -> ElementBuilder;
    fn build(self) -> ElementWithChildrenRecipe;
}

pub struct ElementBuilder {
    key: Option<u64>,
    kind: TagName,
    attributes: BTreeMap<AttributeName, AttributeValue>,
    text: Option<AttributeValue>,
    input_value: Option<AttributeValue>,
    children: Vec<ElementBuilder>,
    event_listeners: Vec<ElementAddEventListenerMutation>,
}

pub fn calculate_hash<T: std::hash::Hash>(t: &T) -> u64 {
    let mut s = std::collections::hash_map::DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[derive(Debug)]
pub struct ElementRecipe {
    pub key: Option<u64>,
    pub create: ElementCreateMutation,
    pub modify: Vec<ElementBuilderModifyMutation>,
    pub finish: ElementFinishMutation,
}

#[derive(Debug)]
pub struct ElementWithChildrenRecipe {
    pub element: ElementRecipe,
    pub children: Vec<Self>,
}

macro_rules! cached {
    ($value:expr) => {
        value_cache::VALUE_CACHE.with(|value_cache| {
            value_cache
                .try_borrow_mut()
                .ok()
                .map(|mut value_cache| value_cache.cached($value))
                .unwrap_or_else(|| $value.into_value())
        })
    };
}

impl<'a> From<&'a str> for ElementBuilder {
    fn from(val: &'a str) -> Self {
        ElementBuilder::builder(val)
    }
}

impl<'a> AsElementBuilder for &'a str {
    fn into_builder(self) -> ElementBuilder {
        ElementBuilder::from(self).into_builder()
    }

    fn text(self, text: impl IntoJsValue) -> ElementBuilder {
        ElementBuilder::from(self).text(text)
    }

    fn input_value(self, value: impl IntoJsValue) -> ElementBuilder {
        ElementBuilder::from(self).input_value(value)
    }

    fn event<Key: std::hash::Hash, EventKind>(
        self,
        key: Key,
        name: impl IntoJsValue,
        callback: impl Fn(EventKind) + 'static,
    ) -> ElementBuilder
    where
        EventKind: std::fmt::Debug + Sized + RefFromWasmAbi + FromWasmAbi + 'static,
        by_event_kind::ElementAddEventListenerMutation<EventKind>:
            Into<ElementAddEventListenerMutation> + Perform,
    {
        ElementBuilder::from(self).event(key, name, callback)
    }

    fn child(self, child: impl Into<ElementBuilder>) -> ElementBuilder {
        ElementBuilder::from(self).child(child.into())
    }

    fn key(self, key: impl std::hash::Hash) -> ElementBuilder {
        ElementBuilder::from(self).key(key)
    }

    fn children(
        self,
        children: impl IntoIterator<Item = impl Into<ElementBuilder>>,
    ) -> ElementBuilder {
        ElementBuilder::from(self).children(children)
    }

    fn attribute(self, attribute: impl IntoJsValue, value: impl IntoJsValue) -> ElementBuilder {
        ElementBuilder::from(self).attribute(attribute, value)
    }

    fn build(self) -> ElementWithChildrenRecipe {
        ElementBuilder::from(self).build()
    }
}

impl AsElementBuilder for ElementBuilder {
    fn into_builder(self) -> ElementBuilder {
        self
    }
    fn builder(kind: impl IntoJsValue) -> Self {
        let kind = cached!(kind).into();
        Self {
            key: None,
            kind,
            input_value: None,
            text: None,
            event_listeners: Default::default(),
            attributes: Default::default(),
            children: Default::default(),
        }
    }

    fn text(mut self, text: impl IntoJsValue) -> Self {
        self.text = Some(cached!(text).into());
        self
    }

    fn input_value(mut self, value: impl IntoJsValue) -> Self {
        self.input_value = Some(cached!(value).into());
        self
    }
    fn event<Key: std::hash::Hash, EventKind>(
        mut self,
        key: Key,
        name: impl IntoJsValue,
        callback: impl Fn(EventKind) + 'static,
    ) -> Self
    where
        EventKind: std::fmt::Debug + Sized + RefFromWasmAbi + FromWasmAbi + 'static,
        by_event_kind::ElementAddEventListenerMutation<EventKind>:
            Into<ElementAddEventListenerMutation> + Perform,
    {
        let hash = calculate_hash(&key);
        let listener = EventListenerWrapper::<EventKind> {
            name: cached!(name).into(),
            closure: KorvinClosure {
                hash,
                closure: Arc::new(Closure::new(callback)),
            },
        };
        self.event_listeners
            .push(by_event_kind::ElementAddEventListenerMutation { listener }.into());
        self
    }
    fn child(mut self, child: impl Into<ElementBuilder>) -> Self {
        self.children.push(child.into());
        self
    }
    fn key(mut self, key: impl std::hash::Hash) -> Self {
        self.key = Some(calculate_hash(&key));
        self
    }

    fn children(self, children: impl IntoIterator<Item = impl Into<ElementBuilder>>) -> Self {
        children
            .into_iter()
            .fold(self, |parent, child| parent.child(child))
    }

    fn attribute(mut self, attribute: impl IntoJsValue, value: impl IntoJsValue) -> Self {
        self.attributes
            .insert(cached!(attribute).into(), cached!(value).into());
        self
    }

    fn build(self) -> ElementWithChildrenRecipe {
        let Self {
            key,
            kind,
            attributes,
            children,
            text,
            event_listeners,
            input_value,
        } = self;

        let element = ElementRecipe {
            key,
            create: ElementCreateMutation { kind },
            modify: empty()
                .chain(
                    attributes
                        .into_iter()
                        .map(|(attribute, value)| ElementSetAttributeMutation {
                            attribute,
                            value: Some(value),
                        })
                        .map(ElementBuilderModifyMutation::from),
                )
                .chain(
                    text.into_iter()
                        .map(|value| ElementSetTextMutation { value: Some(value) })
                        .map(ElementBuilderModifyMutation::from),
                )
                .chain(
                    input_value
                        .into_iter()
                        .map(|value| ElementSetInputValueMutation { value })
                        .map(ElementBuilderModifyMutation::from),
                )
                .chain(
                    event_listeners
                        .into_iter()
                        .map(ElementBuilderModifyMutation::from),
                )
                .collect(),
            finish: ElementFinishMutation {},
        };
        ElementWithChildrenRecipe {
            element,
            children: children.into_iter().map(Self::build).collect(),
        }
    }
}
