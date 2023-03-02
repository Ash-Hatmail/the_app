use druid::{Widget, WidgetExt};
use druid::widget::{Label, Flex, TextBox, Button};

use crate::data::TodoState;

pub fn ui_builder() -> impl Widget<TodoState> {
    let header = Flex::row()
        .with_child(TextBox::new().lens(TodoState::new_text))
        .with_child(Button::new("->"));
    
    Flex::column()
}