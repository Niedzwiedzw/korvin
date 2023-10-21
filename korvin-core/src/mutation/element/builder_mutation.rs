use crate::impl_complex_mutation_wrapper;

use self::marker::{
    create::ElementCreateMutation, finish::ElementFinishMutation, ElementBuilderMarkerMutation,
};

pub mod marker;
pub mod modify;

impl AsRef<ElementBuilderMutation> for ElementBuilderMutation {
    fn as_ref(&self) -> &ElementBuilderMutation {
        self
    }
}

impl_complex_mutation_wrapper! {
    reverse = super::cleanup_mutation::Mutation,
    #[derive(Eq, Hash, PartialOrd)]
    enum ElementBuilderMutation {
        Marker(marker::Mutation),
        Modify(modify::Mutation),

    },
    enum ElementBuilderMutationLog {
        Marker(marker::Log),
        Modify(modify::Log),
    }
}

impl TryInto<ElementCreateMutation> for ElementBuilderMutation {
    type Error = &'static str;

    fn try_into(self) -> Result<ElementCreateMutation, Self::Error> {
        TryInto::<ElementBuilderMarkerMutation>::try_into(self).and_then(|marker| marker.try_into())
    }
}
impl TryInto<ElementFinishMutation> for ElementBuilderMutation {
    type Error = &'static str;

    fn try_into(self) -> Result<ElementFinishMutation, Self::Error> {
        TryInto::<ElementBuilderMarkerMutation>::try_into(self).and_then(|marker| marker.try_into())
    }
}
