use crate::{data::ElementId, impl_complex_mutation};

#[derive(Debug, PartialEq, Clone, Eq, Hash, PartialOrd)]
pub struct ElementFinishMutation {}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementFinishMutationLog {
    pub element_id: ElementId,
}

impl_complex_mutation! {
    mutation = ElementFinishMutation,
    log = ElementFinishMutationLog,
    reverse = super::super::super::cleanup_mutation::marker::unfinish::Mutation,
    fn perform(&self, element_id: crate::data::ElementId) -> crate::mutation::error::MutationResult<Self::Log> {
        Ok(Self::Log { element_id })
    },
    fn revert(&self) -> Self::Mutation {
        let Self { element_id } = self.clone();
        Self::Mutation { element_id }
    }
}
