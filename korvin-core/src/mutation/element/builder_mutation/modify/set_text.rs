use crate::{data::AttributeValue, impl_complex_mutation, raw_operations};

#[derive(Debug, PartialEq, Clone, Eq, Hash, PartialOrd)]
pub struct ElementSetTextMutation {
    pub value: Option<AttributeValue>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementSetTextMutationLog {
    pub previous_value: Option<AttributeValue>,
}

impl_complex_mutation! {
    mutation = ElementSetTextMutation,
    log = ElementSetTextMutationLog,
    reverse = super::super::super::cleanup_mutation::modify::unset_text::Mutation,
    fn perform(&self, element: crate::data::ElementId) -> crate::mutation::error::MutationResult<Self::Log> {
        let Self { value } = self.clone();
        Ok(raw_operations::set_text(element, value))
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
