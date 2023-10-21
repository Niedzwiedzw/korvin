use crate::{
    data::{AttributeName, AttributeValue},
    impl_complex_mutation,
    mutation::error::MutationError,
    raw_operations,
};

#[derive(Debug, PartialEq, Clone)]
pub struct ElementUnsetAttributeMutation {
    pub attribute: AttributeName,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementUnsetAttributeMutationLog {
    pub attribute: AttributeName,
    pub previous_value: AttributeValue,
}

impl_complex_mutation! {
    mutation = ElementUnsetAttributeMutation,
    log = ElementUnsetAttributeMutationLog,
    reverse = super::set_attribute::Mutation,
    fn perform(&self, state: &mut crate::DocumentModel) -> crate::mutation::error::MutationResult<Self::Log> {
        let Self { attribute } = self.clone();
        let element = state.current_root().clone();

        raw_operations::unset_attribute(element, attribute)
            .map_err(MutationError::UnsetAttribute)
            .map(|(attribute, previous_value)| {
                Self::Log { attribute, previous_value }
            })
    },
    fn revert(&self) -> Self::Mutation {
        let Self { attribute, previous_value } = self.clone();
        Self::Mutation { attribute, value: previous_value }
    }
}
