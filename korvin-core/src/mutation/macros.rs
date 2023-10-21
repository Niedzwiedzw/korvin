#[macro_export]
macro_rules! impl_complex_mutation {
    (mutation = $mutation:ty, log = $log:ty, reverse = $reverse:ty, $perform:item, $revert:item) => {
        pub type Mutation = $mutation;
        pub type Log = $log;

        impl $crate::mutation::traits::Perform for $mutation {
            type Log = $log;
            fn to_mutation(self) -> $crate::mutation::Mutation {
                $crate::mutation::traits::Perform::to_mutation(super::Mutation::from(self))
            }
            $perform
        }

        impl $crate::mutation::traits::Revert for $log {
            type Mutation = $reverse;
            fn to_mutation_log(self) -> $crate::mutation::MutationLog {
                $crate::mutation::traits::Revert::to_mutation_log(super::Log::from(self))
            }
            $revert
        }
    }
}

#[macro_export]
macro_rules! impl_mutation {
    (mutation = $mutation:ty, log = $log:ty, $perform:item, $revert:item) => {
        $crate::impl_complex_mutation! {
            mutation = $mutation,
            log = $log,
            reverse = $mutation,
            $perform,
            $revert
        }
    };
}

#[macro_export]
macro_rules! impl_complex_mutation_wrapper {
    (
        reverse = $reverse_mutation:ty,
        $(#[$mutation_meta:meta])* enum $mutation:ident {
            $(
                $(#[$mutation_variant_meta:meta])*
                $mutation_variant:ident($mutation_inner:ty)
            ),* $(,)?
        },
        $(#[$log_meta:meta])* enum $mutation_log:ident {
            $(
                $(#[$log_variant_meta:meta])*
                $log_variant:ident($log_inner:ty)
            ),* $(,)?
        }
    ) => {
        $(#[$mutation_meta])*
        #[derive(Debug, derive_more::From, PartialEq, Clone)]
        #[derive(derive_more::TryInto)]
        #[try_into(owned, ref, ref_mut)]
        pub enum $mutation {
            $(
                $(#[$mutation_variant_meta])*
                $mutation_variant($mutation_inner)
            ),*
        }

        $(#[$log_meta])*
        #[derive(Debug, derive_more::From, PartialEq, Clone)]
        #[derive(derive_more::TryInto)]
        #[try_into(owned, ref, ref_mut)]
        pub enum $mutation_log{
            $(
                $(#[$log_variant_meta])*
                $log_variant($log_inner)
            ),*
        }

        $crate::impl_complex_mutation! {
            mutation = $mutation,
            log = $mutation_log,
            reverse = $reverse_mutation,
            fn perform(&self, state: $crate::data::ElementId) -> $crate::mutation::error::MutationResult<Self::Log> {
                match self {
                    $(Self::$mutation_variant(inner) => inner.perform(state).map(Into::into)),*
                }
            },
            fn revert(&self) -> Self::Mutation {
                match self {
                    $(Self::$log_variant(inner) => inner.revert().into()),*
                }
            }
        }
    }
}

#[macro_export]
macro_rules! impl_mutation_wrapper {
    (
        $(#[$mutation_meta:meta])* enum $mutation:ident {
            $(
                $(#[$mutation_variant_meta:meta])*
                $mutation_variant:ident($mutation_inner:ty)
            ),* $(,)?
        },
        $(#[$log_meta:meta])* enum $mutation_log:ident {
            $(
                $(#[$log_variant_meta:meta])*
                $log_variant:ident($log_inner:ty)
            ),* $(,)?
        }
    ) => {
        $(#[$mutation_meta])*
        #[derive(Debug, derive_more::From, PartialEq, Clone)]
        pub enum $mutation {
            $(
                $(#[$mutation_variant_meta])*
                $mutation_variant($mutation_inner)
            ),*
        }

        $(#[$log_meta])*
        #[derive(Debug, derive_more::From, PartialEq, Clone)]
        pub enum $mutation_log{
            $(
                $(#[$log_variant_meta])*
                $log_variant($log_inner)
            ),*
        }

        $crate::impl_mutation! {
            mutation = $mutation,
            log = $mutation_log,
            fn perform(&self, state: $crate::data::ElementId) -> $crate::mutation::error::MutationResult<Self::Log> {
                match self {
                    $(Self::$mutation_variant(inner) => inner.perform(state).map(Into::into)),*
                }
            },
            fn revert(&self) -> Self::Mutation {
                match self {
                    $(Self::$log_variant(inner) => inner.revert().into()),*
                }
            }
        }
    }
}
