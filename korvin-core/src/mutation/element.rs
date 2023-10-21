use crate::impl_mutation_wrapper;

pub mod builder_mutation;
pub mod cleanup_mutation;

impl_mutation_wrapper! {
    enum ElementMutation {
        Builder(builder_mutation::Mutation),
        Cleanup(cleanup_mutation::Mutation)
    },
    enum ElementMutationLog {
        Builder(builder_mutation::Log),
        Cleanup(cleanup_mutation::Log),
    }
}
