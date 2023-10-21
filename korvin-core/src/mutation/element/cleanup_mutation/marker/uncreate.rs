use crate::{data::TagName, impl_complex_mutation, raw_operations};

#[derive(Debug, PartialEq, Clone)]
pub struct ElementUncreateMutation {
    pub kind: TagName,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementUncreateMutationLog {
    pub kind: TagName,
}

impl_complex_mutation! {
    mutation = ElementUncreateMutation,
    log = ElementUncreateMutationLog,
    reverse = super::super::super::builder_mutation::marker::create::Mutation,
    fn perform(&self, current_root: crate::data::ElementId) -> crate::mutation::error::MutationResult<Self::Log> {
        let Self { kind } = self.clone();
        raw_operations::remove_element_in_place(current_root);
        Ok(Self::Log { kind })
    },
    fn revert(&self) -> Self::Mutation {
        let Self { kind } = self.clone();
        Self::Mutation { kind }
    }
}
