use crate::impl_complex_mutation_wrapper;

pub mod marker;
pub mod modify;

impl_complex_mutation_wrapper! {
    reverse = super::builder_mutation::Mutation,
    enum ElementCleanupMutation {
        Marker(marker::Mutation),
        Modify(modify::Mutation),
    },
    enum ElementCleanupMutationLog {
        Marker(marker::Log),
        Modify(modify::Log),
    }
}
