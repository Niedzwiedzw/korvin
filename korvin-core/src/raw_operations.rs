use self::error::{DebugOf, JsError, RawOperationError, RawOperationResult};
use crate::data::{AttributeName, AttributeValue, ElementId, EventListenerWrapper, TagName};
use tracing::instrument;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlInputElement, Node};

pub mod attribute;
pub mod error;

#[derive(Debug)]
pub struct Removed<T>(T);

impl<T> Removed<T> {
    pub fn unpack(self) -> T {
        self.0
    }
}

#[instrument(level = "trace", ret)]
pub fn remove_element_in_place(element_id: ElementId) -> Removed<ElementId> {
    element_id.as_ref().remove();
    Removed(element_id)
}

#[instrument(level = "trace", ret, err)]
pub fn pick_up_children(from: ElementId) -> RawOperationResult<Vec<Node>> {
    let parent_nodes = from.as_ref().child_nodes();
    (0..parent_nodes.length())
        .rev()
        .filter_map(|node| parent_nodes.get(node))
        .map(|node| from.as_ref().remove_child(&node).map_err(JsError::from))
        .collect::<Result<Vec<_>, _>>()
        .map(|mut children| {
            children.reverse();
            children
        })
        .map_err(|source| RawOperationError::PickingUpChildren {
            from_parent: DebugOf::new(&from),
            source,
        })
}

#[instrument(level = "trace", ret, err)]
pub fn place_children(to: ElementId, children: Vec<Node>) -> RawOperationResult<()> {
    children
        .into_iter()
        .try_for_each(|node| {
            to.as_ref()
                .append_child(&node)
                .map(|_| ())
                .map_err(JsError::from)
        })
        .map_err(|source| RawOperationError::InsertingChildren {
            to_parent: DebugOf::new(&to),
            source,
        })
}

#[instrument(level = "trace", ret, err)]
pub fn reparent_all(from: ElementId, to: ElementId) -> RawOperationResult<()> {
    pick_up_children(from).and_then(|picked_up| place_children(to, picked_up))
}

#[instrument(level = "trace", ret, err)]
pub fn swap_siblings(node_1: ElementId, node_2: ElementId) -> RawOperationResult<()> {
    node_1
        .as_ref()
        .before_with_node_1(node_2.as_ref())
        .map_err(JsError::from)
        .map_err(|source| RawOperationError::SwappingElements {
            element: DebugOf::new(&node_1),
            with: DebugOf::new(&node_2),
            source,
        })
}

#[instrument(level = "trace", ret, err)]
pub fn replace_element(element: ElementId, with: ElementId) -> RawOperationResult<ElementId> {
    element
        .as_ref()
        .replace_with_with_node_1(with.as_ref())
        .map_err(JsError::from)
        .map_err(|source| RawOperationError::SwappingElements {
            element: DebugOf::new(&element),
            with: DebugOf::new(&with),
            source,
        })
        .map(|_| with)
}

#[instrument(level = "trace", ret, err)]
pub fn create_element(document: &Document, kind: TagName) -> RawOperationResult<ElementId> {
    document
        .create_element(kind.as_ref())
        .map_err(JsError::from)
        .map_err(|source| RawOperationError::CreatingElement { kind, source })
        .map(ElementId::new)
}

pub(crate) fn add_event_listener<EventKind>(
    element: ElementId,
    event_listener: EventListenerWrapper<EventKind>,
) -> RawOperationResult<EventListenerWrapper<EventKind>> {
    element
        .as_ref()
        .add_event_listener_with_callback(
            event_listener.name.as_ref(),
            event_listener.closure.js_function(),
        )
        .map_err(JsError::from)
        .map_err(RawOperationError::AddEventListener)
        .map(|_| event_listener)
}

pub(crate) fn remove_event_listener<EventKind>(
    element: ElementId,
    event_listener: EventListenerWrapper<EventKind>,
) -> RawOperationResult<EventListenerWrapper<EventKind>> {
    element
        .as_ref()
        .remove_event_listener_with_callback(
            event_listener.name.as_ref(),
            event_listener.closure.js_function(),
        )
        .map_err(JsError::from)
        .map_err(RawOperationError::RemoveEventListener)
        .map(|_| event_listener)
}

#[instrument(level = "trace", ret, err)]
pub fn insert_element(element: ElementId, to: ElementId) -> RawOperationResult<ElementId> {
    let inserted = element.clone();
    to.as_ref()
        .append_child(element.as_ref())
        .map_err(JsError::from)
        .map_err(|source| RawOperationError::InsertElement {
            to: DebugOf::new(&to),
            element: DebugOf::new(&element),
            source,
        })
        .map(|_| inserted)
}

#[instrument(level = "trace", ret, err)]
pub fn set_attribute(
    element: ElementId,
    attribute: AttributeName,
    value: Option<AttributeValue>,
) -> RawOperationResult<(AttributeName, Option<AttributeValue>)> {
    let old = element
        .as_ref()
        .get_attribute(attribute.as_ref())
        .map(From::from);
    match value {
        Some(value) => element
            .as_ref()
            .set_attribute(attribute.as_ref(), value.as_ref())
            .map_err(JsError::from)
            .map_err(|source| RawOperationError::SetAttribute {
                element: DebugOf::new(&element),
                attribute: attribute.clone(),
                value: Some(value.clone()),
                source,
            })
            .map(|_| (attribute.clone(), old)),
        None => element
            .as_ref()
            .remove_attribute(attribute.as_ref())
            .map_err(JsError::from)
            .map_err(|source| RawOperationError::SetAttribute {
                element: DebugOf::new(&element),
                attribute: attribute.clone(),
                value: None,
                source,
            })
            .map(|_| (attribute.clone(), old)),
    }
}

#[instrument(level = "trace", ret, err)]
pub fn remove_element(from_parent: ElementId, element: ElementId) -> RawOperationResult<ElementId> {
    from_parent
        .as_ref()
        .remove_child(element.as_ref())
        .map_err(JsError::from)
        .map_err(|source| RawOperationError::RemoveElement {
            from_parent: DebugOf::new(&from_parent),
            element: DebugOf::new(&element),
            source,
        })
        .and_then(
            #[allow(unused_variables)]
            {
                |node| -> RawOperationResult<_> {
                    #[cfg(debug_assertions)]
                    {
                        if (&node) != (element.as_ref().as_ref()) {
                            return Err(RawOperationError::ElementMismatch {
                                expected: DebugOf::new(&element),
                                got: DebugOf::new(&node),
                            });
                        }
                    }

                    Ok(element)
                }
            },
        )
}

#[instrument(level = "trace", ret)]
pub fn set_text(element: ElementId, text: Option<AttributeValue>) -> Option<AttributeValue> {
    let old = element.as_ref().text_content().map(AttributeValue::from);
    element
        .as_ref()
        .set_text_content(text.as_ref().map(|a| a.as_ref()));
    old
}

#[instrument(level = "trace", ret)]
pub fn set_input_value(
    element: ElementId,
    value: AttributeValue,
) -> RawOperationResult<AttributeValue> {
    element
        .as_ref()
        .clone()
        .dyn_into::<HtmlInputElement>()
        .map_err(|actual| RawOperationError::NotAnInputElement {
            element: DebugOf::new(&actual),
        })
        .map(|input| {
            let old = input.value();
            input.set_value(value.as_ref());
            old.into()
        })
}

#[instrument(level = "trace", ret)]
pub fn unset_text(element: ElementId) -> Option<AttributeValue> {
    let old = element.as_ref().text_content().map(AttributeValue::from);
    element.as_ref().set_text_content(None);
    old
}
