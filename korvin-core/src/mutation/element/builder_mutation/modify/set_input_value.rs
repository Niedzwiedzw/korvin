use crate::{
    data::AttributeValue, impl_complex_mutation, mutation::error::MutationError, raw_operations,
};

#[derive(Debug, PartialEq, Clone, Eq, Hash, PartialOrd)]
pub struct ElementSetInputValueMutation {
    pub value: AttributeValue,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementSetInputValueMutationLog {
    pub previous_value: AttributeValue,
}

impl_complex_mutation! {
    mutation = ElementSetInputValueMutation,
    log = ElementSetInputValueMutationLog,
    reverse = super::super::super::cleanup_mutation::modify::unset_input_value::Mutation,
    fn perform(&self, element: crate::data::ElementId) -> crate::mutation::error::MutationResult<Self::Log> {
        let Self { value } = self.clone();
        raw_operations::set_input_value(element, value).map_err(MutationError::SetInputValue)
            .map(|previous_value| {
                Self::Log { previous_value }
            })
    },
    fn revert(&self) -> Self::Mutation {
        let Self { previous_value } = self.clone();
        Self::Mutation {
            previous_value
        }
    }
}
