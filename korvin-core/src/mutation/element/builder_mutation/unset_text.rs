use crate::{
    data::AttributeValue, impl_complex_mutation, mutation::error::MutationError, raw_operations,
};

#[derive(Debug, PartialEq, Clone)]
pub struct ElementUnsetTextMutation {}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementUnsetTextMutationLog {
    pub previous_value: AttributeValue,
}

impl_complex_mutation! {
    mutation = ElementUnsetTextMutation,
    log = ElementUnsetTextMutationLog,
    reverse = super::set_text::Mutation,
    fn perform(&self, state: &mut crate::DocumentModel) -> crate::mutation::error::MutationResult<Self::Log> {
        let element = state.current_root().clone();

        raw_operations::unset_text(element)
            .map_err(MutationError::UnsetAttribute)
            .map(|previous_value| {
                Self::Log { previous_value }
            })
    },
    fn revert(&self) -> Self::Mutation {
        let Self { previous_value } = self.clone();
        Self::Mutation { value: previous_value }
    }
}
