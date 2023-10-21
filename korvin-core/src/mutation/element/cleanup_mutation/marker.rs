pub mod uncreate;
pub mod unfinish;
use crate::impl_complex_mutation_wrapper;

impl_complex_mutation_wrapper! {
    reverse = super::super::builder_mutation::marker::Mutation,
    enum ElementCleanupMutation {
        Uncreate(uncreate::Mutation),
        Unfinish(unfinish::Mutation),
    },
    enum ElementCleanupMutationLog {
        Uncreate(uncreate::Log),
        Unfinish(unfinish::Log),
    }
}
