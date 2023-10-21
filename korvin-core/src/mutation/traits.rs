use crate::data::ElementId;

use super::error::MutationResult;
use std::fmt::Debug;

pub trait Revert: Clone + PartialEq {
    type Mutation;
    fn revert(&self) -> Self::Mutation;
    fn to_mutation_log(self) -> super::MutationLog;
}

pub trait Perform: Clone + Debug {
    type Log;
    fn perform(&self, state: ElementId) -> MutationResult<Self::Log>;
    fn to_mutation(self) -> super::Mutation;
}
