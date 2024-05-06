use iced::event::{self, Event};
use iced::widget::{
    button, center, checkbox, container, list, scrollable, text, Column,
    Scrollable,
};
use iced::window;
use iced::{Center, Element, Fill, Font, Subscription, Task};

pub fn main() -> iced::Result {
    iced::application("Events - Iced", Events::update, Events::view)
        .subscription(Events::subscription)
        .exit_on_close_request(false)
        .run()
}

#[derive(Debug, Default)]
struct Events {
    log: list::Content<Event>,
    enabled: bool,
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
    Toggled(bool),
    Exit,
}

impl Events {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EventOccurred(event) if self.enabled => {
                self.log.push(event);

                if self.log.len() > 1_000 {
                    self.log.remove(0);
                }

                Task::none()
            }
            Message::EventOccurred(event) => {
                if let Event::Window(window::Event::CloseRequested) = event {
                    window::get_latest().and_then(window::close)
                } else {
                    Task::none()
                }
            }
            Message::Toggled(enabled) => {
                self.enabled = enabled;

                Task::none()
            }
            Message::Exit => window::get_latest().and_then(window::close),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::EventOccurred)
    }

    fn view(&self) -> Element<Message> {
        let events = container(
            scrollable(
                container(list(&self.log, |_i, event| {
                    text!("{event:?}")
                        .size(14)
                        .font(Font::MONOSPACE)
                        .into()
                }))
                .padding(10),
            )
            .height(Fill),
        )
        .style(container::rounded_box)
        .padding(5);

        let toggle = checkbox("Listen to runtime events", self.enabled)
            .on_toggle(Message::Toggled);

        let exit = button(text("Exit").width(Fill).align_x(Center))
            .width(100)
            .padding(10)
            .on_press(Message::Exit);

        let content = Column::new()
            .align_x(Center)
            .spacing(20)
            .push(events)
            .push(toggle)
            .push(exit);

        center(content).padding(10).into()
    }
}
