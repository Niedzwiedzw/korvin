use crate::{data::KorvinClosure, impl_complex_mutation};

#[derive(Debug, PartialEq, Clone, Eq, Hash, PartialOrd)]
pub struct ElementRunOnMountedMutation {
    on_mounted: KorvinClosure<()>,
    on_unmounted: KorvinClosure<()>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementRunOnMountedMutationLog {
    on_mounted: KorvinClosure<()>,
    on_unmounted: KorvinClosure<()>,
}

impl_complex_mutation! {
    mutation = ElementRunOnMountedMutation,
    log = ElementRunOnMountedMutationLog,
    reverse = super::super::super::cleanup_mutation::modify::run_on_unmounted::Mutation,
    fn perform(&self, element: crate::data::ElementId) -> crate::mutation::error::MutationResult<Self::Log> {
        todo!()
    },
    fn revert(&self) -> Self::Mutation {
        todo!()
    }
}
