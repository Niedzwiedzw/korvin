pub mod create;
pub mod finish;
use crate::impl_complex_mutation_wrapper;

impl_complex_mutation_wrapper! {
    reverse = super::super::cleanup_mutation::marker::Mutation,
    #[derive(Eq, Hash, PartialOrd)]
    enum ElementBuilderMarkerMutation {
        Create(create::Mutation),
        Finish(finish::Mutation),
    },
    enum ElementBuilderMarkerMutationLog {
        Create(create::Log),
        Finish(finish::Log),
    }
}
