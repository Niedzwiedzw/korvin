use crate::{
    data::AttributeValue, impl_complex_mutation, mutation::error::MutationError, raw_operations,
};

#[derive(Debug, PartialEq, Clone)]
pub struct ElementUnsetInputValueMutation {
    pub previous_value: AttributeValue,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementUnsetInputValueMutationLog {
    pub previous_value: AttributeValue,
}

impl_complex_mutation! {
    mutation = ElementUnsetInputValueMutation,
    log = ElementUnsetInputValueMutationLog,
    reverse = super::super::super::builder_mutation::modify::set_input_value::Mutation,
    fn perform(&self, state: crate::data::ElementId) -> crate::mutation::error::MutationResult<Self::Log> {
        let Self { previous_value } = self.clone();
        let element = state;
        let previous_value = raw_operations::set_input_value(element, previous_value).map_err(MutationError::SetInputValue)?;
        Ok(Self::Log { previous_value })
    },
    fn revert(&self) -> Self::Mutation {
        let Self { previous_value } = self.clone();
        Self::Mutation { value: previous_value }
    }
}
