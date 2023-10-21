use crate::{
    data::{ElementId, TagName},
    impl_complex_mutation,
    mutation::{element::builder_mutation::ElementBuilderMutation, error::MutationError},
    raw_operations,
};

#[derive(Debug, PartialEq, Clone, Eq, Hash, PartialOrd)]
pub struct ElementCreateMutation {
    pub kind: TagName,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementCreateMutationLog {
    pub kind: TagName,
    pub element_id: ElementId,
}

impl_complex_mutation! {
    mutation = ElementCreateMutation,
    log = ElementCreateMutationLog,
    reverse = super::super::super::cleanup_mutation::marker::uncreate::Mutation,
    fn perform(&self, parent: crate::data::ElementId) -> crate::mutation::error::MutationResult<Self::Log> {
        let Self { kind } = self.clone();
        crate::DOCUMENT.with(|document| {
            raw_operations::create_element(document, kind.clone())
                .and_then(|element| raw_operations::insert_element(element, parent.clone()))
                .map_err(MutationError::ElementCreate)
                .map(|inserted| {
                    Self::Log { kind, element_id: inserted }
                })
        })

    },
    fn revert(&self) -> Self::Mutation {
        let Self { kind, element_id: _ } = self.clone();
        Self::Mutation { kind }
    }
}

impl From<ElementCreateMutation> for ElementBuilderMutation {
    fn from(value: ElementCreateMutation) -> Self {
        Self::Marker(value.into())
    }
}
