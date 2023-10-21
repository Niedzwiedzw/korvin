use crate::impl_complex_mutation_wrapper;
pub mod remove_event_listener;
pub mod unset_attribute;
pub mod unset_input_value;
pub mod unset_text;

impl_complex_mutation_wrapper! {
    reverse = super::super::builder_mutation::modify::Mutation,
    enum ElementCleanupModifyMutation {
        UnsetAttribute(unset_attribute::Mutation),
        RemoveEventListener(remove_event_listener::Mutation),
        UnsetText(unset_text::Mutation),
        UnsetInputValue(unset_input_value::Mutation),
    },
    enum ElementCleanupModifyMutationLog {
        UnsetAttribute(unset_attribute::Log),
        RemoveEventListener(remove_event_listener::Log),
        UnsetText(unset_text::Log),
        UnsetInputValue(unset_input_value::Log),
    }
}
