use crate::{
    data::{AttributeName, AttributeValue},
    impl_complex_mutation,
    mutation::error::MutationError,
    raw_operations,
};

#[derive(Debug, PartialEq, Clone, Eq, Hash, PartialOrd)]
pub struct ElementSetAttributeMutation {
    pub attribute: AttributeName,
    pub value: Option<AttributeValue>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementSetAttributeMutationLog {
    pub attribute: AttributeName,
    pub previous_value: Option<AttributeValue>,
}

impl_complex_mutation! {
    mutation = ElementSetAttributeMutation,
    log = ElementSetAttributeMutationLog,
    reverse = super::super::super::cleanup_mutation::modify::unset_attribute::Mutation,
    fn perform(&self, element: crate::data::ElementId) -> crate::mutation::error::MutationResult<Self::Log> {
        let Self { attribute, value } = self.clone();

        raw_operations::set_attribute(element, attribute, value)
            .map_err(MutationError::SetAttribute)
            .map(|(attribute, previous_value)| {
                Self::Log { attribute, previous_value }
            })
    },
    fn revert(&self) -> Self::Mutation {
        let Self { attribute, previous_value } = self.clone();
        crate::mutation::element::cleanup_mutation::modify::unset_attribute::ElementUnsetAttributeMutation { attribute, previous_value }
    }
}
