use crate::{data::KorvinClosure, impl_complex_mutation};

#[derive(Debug, PartialEq, Clone)]
pub struct ElementRunOnUnmountedMutation {
    on_mounted: KorvinClosure<()>,
    on_unmounted: KorvinClosure<()>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementRunOnUnmountedMutationLog {
    on_mounted: KorvinClosure<()>,
    on_unmounted: KorvinClosure<()>,
}

impl_complex_mutation! {
    mutation = ElementRunOnUnmountedMutation,
    log = ElementRunOnUnmountedMutationLog,
    reverse = super::super::super::builder_mutation::modify::run_on_mounted::Mutation,
    fn perform(&self, element: crate::data::ElementId) -> crate::mutation::error::MutationResult<Self::Log> {
        todo!()
    },
    fn revert(&self) -> Self::Mutation {
        todo!()
    }
}
