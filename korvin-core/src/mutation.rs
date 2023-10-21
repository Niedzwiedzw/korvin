pub mod element;
pub mod error;
pub mod traits;
#[macro_use]
pub mod macros;

#[macro_export]
macro_rules! noop_mutation {
    () => {
        pub mod noop {
            use $crate::impl_mutation;

            #[derive(Debug, PartialEq , Clone)]
            pub struct NoopMutation {}

            #[derive(Debug, PartialEq, Clone)]
            pub struct NoopMutationLog {}

            impl_mutation! {
                mutation = NoopMutation,
                log = NoopMutationLog,
                fn perform(&self, _state: &mut $crate::DocumentModel) -> $crate::mutation::error::MutationResult<Self::Log> {
                    Ok(Self::Log {})
                },
                fn revert(&self) -> Self::Mutation {
                    Self::Mutation {}
                }
            }
        }
    }
}

#[derive(Debug, derive_more::From, PartialEq, Clone)]
pub enum Mutation {
    Element(element::Mutation),
}

impl traits::Perform for Mutation {
    type Log = self::MutationLog;

    fn perform(&self, element: crate::data::ElementId) -> error::MutationResult<Self::Log> {
        match self {
            Mutation::Element(i) => i.perform(element).map(Into::into),
        }
    }

    fn to_mutation(self) -> self::Mutation {
        self
    }
}

#[derive(Debug, derive_more::From, Clone, PartialEq)]
pub enum MutationLog {
    Element(element::Log),
}
pub type Log = MutationLog;

impl traits::Revert for MutationLog {
    type Mutation = self::Mutation;

    fn revert(&self) -> Self::Mutation {
        match self {
            MutationLog::Element(i) => i.revert().into(),
        }
    }

    fn to_mutation_log(self) -> self::MutationLog {
        self
    }
}
