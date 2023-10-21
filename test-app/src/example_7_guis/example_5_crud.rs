use super::*;
use korvin_core::{
    element_builder::ElementBuilder,
    web_sys::{InputEvent, MouseEvent},
};

pub enum CrudMessage {
    CreateUser(User),
    DeleteUser(usize),
    UpdateUser(usize, User),
    SetUserForm(UserForm),
    UpdateUserForm(User),
    UpdateFilter(String),
}

#[derive(Default, Debug, Clone, Hash)]
pub struct User {
    first_name: String,
    last_name: String,
}

#[derive(Debug, Clone, Hash)]
pub enum UserForm {
    New(User),
    Edit(usize, User),
}

impl std::default::Default for UserForm {
    fn default() -> Self {
        Self::New(Default::default())
    }
}

#[derive(Default, Debug)]
pub struct Crud {
    filter: String,
    user_form: UserForm,
    users: Vec<User>,
}

impl HandleMessage<CrudMessage> for Crud {
    fn handle(&mut self, message: CrudMessage) {
        match message {
            CrudMessage::CreateUser(user) => {
                self.users.push(user);
                self.user_form = Default::default();
            }
            CrudMessage::DeleteUser(idx) => {
                self.users
                    .get(idx)
                    .map(|_| ())
                    .map(|_| self.users.remove(idx));
            }
            CrudMessage::UpdateUser(idx, user_form) => {
                if let Some(user) = self.users.get_mut(idx) {
                    *user = user_form;
                }
            }
            CrudMessage::SetUserForm(user_form) => {
                self.user_form = user_form;
            }
            CrudMessage::UpdateUserForm(user) => match &mut self.user_form {
                UserForm::New(user_form) => *user_form = user,
                UserForm::Edit(_, user_form) => *user_form = user,
            },
            CrudMessage::UpdateFilter(filter) => {
                self.filter = filter;
            }
        }
    }
}

fn form(
    communicator: Communicator<CrudMessage>,
    user: User,
    message: impl Fn(User) -> CrudMessage + Clone + 'static,
    button_text: &str,
) -> ElementBuilder {
    let first_name = {
        let user = user.clone();
        input(
            ("first_name", user.last_name.clone()),
            communicator,
            user.first_name.clone(),
            move |first_name| {
                CrudMessage::UpdateUserForm(User {
                    first_name: first_name.clone(),
                    ..user.clone()
                })
            },
        )
    };
    let last_name = {
        let user = user.clone();
        input(
            ("last_name", user.first_name.clone()),
            communicator,
            user.last_name.clone(),
            move |last_name| {
                CrudMessage::UpdateUserForm(User {
                    last_name: last_name.clone(),
                    ..user.clone()
                })
            },
        )
    };
    let submit = {
        button((user.clone(), button_text), communicator, move || {
            message(user.clone())
        })
        .text(button_text)
    };
    "div".child(first_name).child(last_name).child(submit)
}

impl ToLazyHtml for WithCommunicator<Crud, CrudMessage> {
    fn to_lazy_html(&self) -> ElementBuilder {
        let Self {
            inner,
            communicator,
        } = self;
        let communicator = *communicator;
        let user_form = {
            let container = "div".attribute("class", "form");

            match inner.user_form.clone() {
                UserForm::New(user) => container.child(
                    form(
                        communicator,
                        user.clone(),
                        CrudMessage::CreateUser,
                        "create",
                    )
                    .key("new"),
                ),
                UserForm::Edit(idx, user) => container
                    .child(
                        form(
                            communicator,
                            user.clone(),
                            move |user| CrudMessage::UpdateUser(idx, user),
                            "update",
                        )
                        .key("edit"),
                    )
                    .child(
                        button("delete", communicator, move || CrudMessage::DeleteUser(idx))
                            .text("delete"),
                    ),
            }
        };
        let filter = {
            "span"
                .child("label".attribute("for", "filter").text("Filter prefix"))
                .child(
                    "input"
                        .input_value(inner.filter.as_str())
                        .attribute("id", "filter")
                        .attribute("name", "filter")
                        .event((), "input", move |e: InputEvent| {
                            e.on_value(communicator, CrudMessage::UpdateFilter)
                        }),
                )
        };
        let filter_user = {
            let filter = inner.filter.clone();
            move |user: &User| -> bool {
                [&user.first_name, &user.last_name]
                    .iter()
                    .any(|v| v.contains(&filter))
            }
        };
        "main"
            .child("h3".text("7 GUIs: Crud"))
            .attribute("class", "crud")
            .child(filter)
            .child(user_form)
            .child(
                "select"
                    .attribute("multiple", "true")
                    .attribute("style", "width: 300px")
                    .children(
                        inner
                            .users
                            .iter()
                            .enumerate()
                            .filter(|(_, user)| filter_user(user))
                            .map(
                                |(
                                    idx,
                                    user @ User {
                                        first_name,
                                        last_name,
                                    },
                                )| {
                                    let display = format!("{last_name}, {first_name}");
                                    let user = user.clone();
                                    "option"
                                        .text(display.as_str())
                                        .attribute("value", display.as_str())
                                        .event((), "mousedown", move |_: MouseEvent| {
                                            communicator.send(CrudMessage::SetUserForm(
                                                UserForm::Edit(idx, user.clone()),
                                            ))
                                        })
                                },
                            ),
                    ),
            )
            .child("div".text(format!("{inner:#?}").as_str()))
    }
}
