use crate::impl_complex_mutation_wrapper;
pub mod add_event_listener;
pub mod set_attribute;
pub mod set_input_value;
pub mod set_text;

impl_complex_mutation_wrapper! {
    reverse = super::super::cleanup_mutation::modify::Mutation,
    #[derive(Eq, Hash, PartialOrd)]
    enum ElementBuilderModifyMutation {
        SetAttribute(set_attribute::Mutation),
        AddEventListener(add_event_listener::Mutation),
        SetText(set_text::Mutation),
        SetInputValue(set_input_value::Mutation),
    },
    enum ElementBuilderModifyMutationLog {
        SetAttribute(set_attribute::Log),
        AddEventListener(add_event_listener::Log),
        SetText(set_text::Log),
        SetInputValue(set_input_value::Log),
    }
}
