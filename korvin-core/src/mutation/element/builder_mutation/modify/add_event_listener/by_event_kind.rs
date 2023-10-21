use crate::{
    data::EventListenerWrapper,
    mutation::{
        element::cleanup_mutation::modify::remove_event_listener::by_event_kind::ElementRemoveEventListenerMutation,
        error::MutationError,
    },
    raw_operations,
};

pub type Mutation<E> = ElementAddEventListenerMutation<E>;
pub type Log<E> = ElementAddEventListenerMutationLog<E>;

#[derive(Debug)]
pub struct ElementAddEventListenerMutation<EventKind> {
    pub listener: EventListenerWrapper<EventKind>,
}

impl<E> PartialEq for ElementAddEventListenerMutation<E> {
    fn eq(&self, other: &Self) -> bool {
        self.listener.eq(&other.listener)
    }
}

impl<E> PartialOrd for ElementAddEventListenerMutation<E> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.listener.partial_cmp(&other.listener)
    }
}

impl<E> Eq for ElementAddEventListenerMutation<E> {}
impl<E> std::hash::Hash for ElementAddEventListenerMutation<E> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.listener.hash(state)
    }
}

impl<E> PartialEq for ElementAddEventListenerMutationLog<E> {
    fn eq(&self, other: &Self) -> bool {
        self.listener.eq(&other.listener)
    }
}

impl<E> Clone for ElementAddEventListenerMutation<E> {
    fn clone(&self) -> Self {
        Self {
            listener: self.listener.clone(),
        }
    }
}

impl<E> Clone for ElementAddEventListenerMutationLog<E> {
    fn clone(&self) -> Self {
        Self {
            listener: self.listener.clone(),
        }
    }
}

#[derive(Debug)]
pub struct ElementAddEventListenerMutationLog<EventKind>
where
    EventKind: 'static,
{
    listener: EventListenerWrapper<EventKind>,
}

impl<EventKind: std::fmt::Debug> crate::mutation::traits::Perform
    for ElementAddEventListenerMutation<EventKind>
where
    Self: Into<super::Mutation>,
    EventKind: 'static,
{
    type Log = ElementAddEventListenerMutationLog<EventKind>;
    fn to_mutation(self) -> crate::mutation::Mutation {
        self.into().to_mutation()
    }
    fn perform(
        &self,
        element: crate::data::ElementId,
    ) -> crate::mutation::error::MutationResult<Self::Log> {
        let Self { listener } = self.clone();
        raw_operations::add_event_listener(element, listener)
            .map_err(MutationError::ElementAddEventListener)
            .map(|listener| Self::Log { listener })
    }
}

impl<EventKind> crate::mutation::traits::Revert for ElementAddEventListenerMutationLog<EventKind>
where
    Self: Into<super::Log>,
{
    type Mutation = ElementRemoveEventListenerMutation<EventKind>;
    fn to_mutation_log(self) -> crate::mutation::MutationLog {
        self.into().to_mutation_log()
    }
    fn revert(&self) -> Self::Mutation {
        let Self { listener } = self.clone();
        Self::Mutation { listener }
    }
}
