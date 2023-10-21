use super::event::{EventName, KorvinClosure};

pub struct EventListenerWrapper<EventKind> {
    pub name: EventName,
    pub closure: KorvinClosure<EventKind>,
}

impl<E> std::hash::Hash for EventListenerWrapper<E> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.closure.hash(state)
    }
}

impl<E> PartialOrd for EventListenerWrapper<E> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.closure.hash.partial_cmp(&other.closure.hash)
    }
}

impl<E> std::clone::Clone for EventListenerWrapper<E> {
    fn clone(&self) -> Self {
        let Self {
            name: kind,
            closure,
        } = self;
        Self {
            name: kind.clone(),
            closure: closure.clone(),
        }
    }
}

impl<E> PartialEq for EventListenerWrapper<E> {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name) && self.closure.eq(&other.closure)
    }
}

impl<E> std::fmt::Debug for EventListenerWrapper<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("kind", &self.name)
            .finish_non_exhaustive()
    }
}
