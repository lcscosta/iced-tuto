use iced::{Length, Settings};
use iced::Sandbox;
use iced::widget::{Container, Column, Text};

struct HelloWorld;

impl Sandbox for HelloWorld {
    type Message = ();

    fn new() -> Self {
        HelloWorld
    }

    fn title(&self) -> String {
        String::from("Hello, world!")
    }

    fn update(&mut self, _message: Self::Message) {
    }

    fn view(&self) -> iced::Element<Self::Message> {
        let label = Text::new("Hello, world!");
        let column = Column::new().push(label);
        Container::new(column).width(Length::Fill).center_x().height(Length::Fill).center_y().into()
    }

}

fn main() -> iced::Result {
    HelloWorld::run(Settings::default())
}
