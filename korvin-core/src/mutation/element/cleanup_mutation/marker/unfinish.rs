use crate::{data::ElementId, impl_complex_mutation};

#[derive(Debug, PartialEq, Clone)]
pub struct ElementUnfinishMutation {
    pub element_id: ElementId,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementUnfinishMutationLog {}

impl_complex_mutation! {
    mutation = ElementUnfinishMutation,
    log = ElementUnfinishMutationLog,
    reverse = super::super::super::builder_mutation::marker::finish::Mutation,
    fn perform(&self, _current_element: crate::data::ElementId) -> crate::mutation::error::MutationResult<Self::Log> {
        Ok(Self::Log {})
    },
    fn revert(&self) -> Self::Mutation {
        Self::Mutation {}
    }
}
