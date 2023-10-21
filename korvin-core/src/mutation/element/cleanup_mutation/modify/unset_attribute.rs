use crate::{
    data::{AttributeName, AttributeValue},
    impl_complex_mutation,
    mutation::error::MutationError,
    raw_operations,
};

#[derive(Debug, PartialEq, Clone)]
pub struct ElementUnsetAttributeMutation {
    pub attribute: AttributeName,
    pub previous_value: Option<AttributeValue>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementUnsetAttributeMutationLog {
    pub attribute: AttributeName,
    pub previous_value: Option<AttributeValue>,
}

impl_complex_mutation! {
    mutation = ElementUnsetAttributeMutation,
    log = ElementUnsetAttributeMutationLog,
    reverse = super::super::super::builder_mutation::modify::set_attribute::Mutation,
    fn perform(&self, root_element: crate::data::ElementId) -> crate::mutation::error::MutationResult<Self::Log> {
        let Self { attribute, previous_value } = self.clone();
        raw_operations::set_attribute(root_element, attribute, previous_value)
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
