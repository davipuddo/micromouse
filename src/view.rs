use crate::lib::{State, Message};

use iced::{
    Element
};

use iced::widget::{
    row, column, text, container,
};

pub fn view(state: &State) -> Element<'_, Message> {
    container(text("Hello World!")).padding(10).into()
}
