use crate::{data::AttributeValue, impl_complex_mutation, raw_operations};

#[derive(Debug, PartialEq, Clone)]
pub struct ElementUnsetTextMutation {
    pub previous_value: Option<AttributeValue>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementUnsetTextMutationLog {
    pub previous_value: Option<AttributeValue>,
}

impl_complex_mutation! {
    mutation = ElementUnsetTextMutation,
    log = ElementUnsetTextMutationLog,
    reverse = super::super::super::builder_mutation::modify::set_text::Mutation,
    fn perform(&self, state: crate::data::ElementId) -> crate::mutation::error::MutationResult<Self::Log> {
        let Self { previous_value } = self.clone();
        let element = state;
        let previous_value = raw_operations::set_text(element, previous_value);
        Ok(Self::Log { previous_value })
    },
    fn revert(&self) -> Self::Mutation {
        let Self { previous_value } = self.clone();
        Self::Mutation { value: previous_value }
    }
}
