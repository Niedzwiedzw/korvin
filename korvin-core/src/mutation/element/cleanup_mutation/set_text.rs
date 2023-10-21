use crate::{data::AttributeValue, impl_complex_mutation, raw_operations};

#[derive(Debug, PartialEq, Clone)]
pub struct ElementSetTextMutation {
    pub value: AttributeValue,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementSetTextMutationLog {
    pub previous_value: Option<AttributeValue>,
}

impl_complex_mutation! {
    mutation = ElementSetTextMutation,
    log = ElementSetTextMutationLog,
    reverse = super::Mutation,
    fn perform(&self, state: &mut crate::DocumentModel) -> crate::mutation::error::MutationResult<Self::Log> {
        let Self { value } = self.clone();
        let element = state.current_root().clone();

        Ok(raw_operations::set_text(element, value))
            .map(|previous_value| {
                Self::Log { previous_value }
            })
    },
    fn revert(&self) -> Self::Mutation {
        let Self { previous_value } = self.clone();
        match previous_value {
            Some(previous_value) => super::ElementMutation::from(self::ElementSetTextMutation {  value: previous_value }),
            None => super::ElementMutation::from(super::unset_text::ElementUnsetTextMutation {})
        }
    }
}
