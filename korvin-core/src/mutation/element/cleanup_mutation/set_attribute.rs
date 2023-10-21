use crate::{
    data::{AttributeName, AttributeValue},
    impl_complex_mutation,
    mutation::error::MutationError,
    raw_operations,
};

#[derive(Debug, PartialEq, Clone)]
pub struct ElementSetAttributeMutation {
    pub attribute: AttributeName,
    pub value: AttributeValue,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementSetAttributeMutationLog {
    pub attribute: AttributeName,
    pub previous_value: Option<AttributeValue>,
}

impl_complex_mutation! {
    mutation = ElementSetAttributeMutation,
    log = ElementSetAttributeMutationLog,
    reverse = super::Mutation,
    fn perform(&self, state: &mut crate::DocumentModel) -> crate::mutation::error::MutationResult<Self::Log> {
        let Self { attribute, value } = self.clone();
        let element = state.current_root().clone();

        raw_operations::set_attribute(element, attribute, value)
            .map_err(MutationError::SetAttribute)
            .map(|(attribute, previous_value)| {
                Self::Log { attribute, previous_value }
            })
    },
    fn revert(&self) -> Self::Mutation {
        let Self { attribute, previous_value } = self.clone();
        match previous_value {
            Some(previous_value) => super::ElementMutation::from(self::ElementSetAttributeMutation { attribute, value: previous_value }),
            None => super::ElementMutation::from(super::unset_attribute::ElementUnsetAttributeMutation { attribute })
        }
    }
}
