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
    reverse = super::finish::Mutation,
    fn perform(&self, state: &mut crate::DocumentModel) -> crate::mutation::error::MutationResult<Self::Log> {
        let Self { element_id } = self.clone();
        state.push_current_root(element_id);

        Ok(Self::Log {})
    },
    fn revert(&self) -> Self::Mutation {
        Self::Mutation {}
    }
}
