use druid::{Date, Lens};
use im::Vector;

#[derive(Clone, Data, Lens, Default)]
pub struct TodoState {
    pub todos: Vector<TodoItem>,
    pub new_text: String,
}

#[derive(Clone, Data, Lens, Default)]
pub struct TodoItem {
    pub checked: bool,
    pub text: String,
}