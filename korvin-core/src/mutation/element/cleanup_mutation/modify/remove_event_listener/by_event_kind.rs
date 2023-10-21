use crate::{
    data::EventListenerWrapper,
    mutation::{
        element::builder_mutation::modify::add_event_listener::by_event_kind::ElementAddEventListenerMutation,
        error::MutationError,
    },
    raw_operations,
};

pub type Mutation<E> = ElementRemoveEventListenerMutation<E>;
pub type Log<E> = ElementRemoveEventListenerMutationLog<E>;

#[derive(Debug)]
pub struct ElementRemoveEventListenerMutation<EventKind> {
    pub listener: EventListenerWrapper<EventKind>,
}

#[derive(Debug)]
pub struct ElementRemoveEventListenerMutationLog<EventKind> {
    listener: EventListenerWrapper<EventKind>,
}
impl<E> std::hash::Hash for ElementRemoveEventListenerMutation<E> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.listener.hash(state)
    }
}

impl<E> Eq for ElementRemoveEventListenerMutation<E> {}

impl<E> PartialEq for ElementRemoveEventListenerMutation<E> {
    fn eq(&self, other: &Self) -> bool {
        self.listener.eq(&other.listener)
    }
}
impl<E> PartialEq for ElementRemoveEventListenerMutationLog<E> {
    fn eq(&self, other: &Self) -> bool {
        self.listener.eq(&other.listener)
    }
}
impl<E> Clone for ElementRemoveEventListenerMutation<E> {
    fn clone(&self) -> Self {
        Self {
            listener: self.listener.clone(),
        }
    }
}
impl<E> Clone for ElementRemoveEventListenerMutationLog<E> {
    fn clone(&self) -> Self {
        Self {
            listener: self.listener.clone(),
        }
    }
}

impl<EventKind> crate::mutation::traits::Perform for ElementRemoveEventListenerMutation<EventKind>
where
    Self: Into<super::Mutation>,
    EventKind: std::fmt::Debug,
{
    type Log = ElementRemoveEventListenerMutationLog<EventKind>;

    fn to_mutation(self) -> crate::mutation::Mutation {
        self.into().to_mutation()
    }

    fn perform(
        &self,
        root_element: crate::data::ElementId,
    ) -> crate::mutation::error::MutationResult<Self::Log> {
        let Self { listener } = self.clone();
        raw_operations::remove_event_listener(root_element, listener)
            .map_err(MutationError::ElementRemoveEventListener)
            .map(|listener| Self::Log { listener })
    }
}

impl<EventKind> crate::mutation::traits::Revert
    for ElementRemoveEventListenerMutationLog<EventKind>
where
    Self: Into<super::Log>,
{
    type Mutation = ElementAddEventListenerMutation<EventKind>;
    fn to_mutation_log(self) -> crate::mutation::MutationLog {
        self.into().to_mutation_log()
    }
    fn revert(&self) -> Self::Mutation {
        let Self { listener } = self.clone();
        Self::Mutation { listener }
    }
}
