//! Tests for how `text_input` and `text_editor` handle the scroll wheel.
//!
//! A single-line `text_input` cannot scroll, so it must leave wheel events
//! uncaptured for an enclosing `scrollable`. A multi-line `text_editor` with
//! bounded height scrolls its own content and keeps capturing them.
use iced_test::Simulator;
use iced_widget::core::mouse::ScrollDelta;
use iced_widget::core::{Point, Widget, event};
use iced_widget::scrollable::Scroll;
use iced_widget::{Renderer, Theme, column, container, scrollable, space, text_editor, text_input};

#[derive(Debug, Clone)]
enum Message {
    Input,
    Scrolled(Scroll),
    Edited,
}

const WHEEL_DOWN: ScrollDelta = ScrollDelta::Lines { x: 0.0, y: -1.0 };

fn simulate<'a>(
    element: impl Widget<Message, Theme, Renderer> + 'a,
) -> Simulator<'a, Message, Theme, Renderer> {
    Simulator::new(element)
}

#[test]
fn text_input_ignores_wheel_when_unfocused() {
    let mut simulator = simulate(text_input("Placeholder", "Value").on_input(|_| Message::Input));
    simulator.point_at(Point::new(10.0, 10.0));

    assert_eq!(simulator.scroll(WHEEL_DOWN), event::Status::Ignored);
}

#[test]
fn text_input_ignores_wheel_when_focused() {
    let mut simulator = simulate(text_input("Placeholder", "Value").on_input(|_| Message::Input));
    simulator.point_at(Point::new(10.0, 10.0));

    let _ = simulator.simulate(iced_test::simulator::click());

    assert_eq!(simulator.scroll(WHEEL_DOWN), event::Status::Ignored);
}

#[test]
fn scrollable_scrolls_with_cursor_over_text_input() {
    // Inside an unbounded scrollable column the input's editor gets infinite
    // bounds and never emits a scroll action. A bounded ancestor height is
    // what exposed the bug.
    let mut simulator = simulate(
        scrollable(column![
            container(text_input("Placeholder", "Value").on_input(|_| Message::Input)).height(40),
            space().height(3000),
        ])
        .height(200)
        .smooth_scroll(false)
        .on_scroll(Message::Scrolled),
    );
    simulator.point_at(Point::new(10.0, 10.0));

    assert_eq!(simulator.scroll(WHEEL_DOWN), event::Status::Captured);

    let offsets: Vec<f32> = simulator
        .into_messages()
        .filter_map(|message| match message {
            Message::Scrolled(scroll) => Some(scroll.viewport.absolute_offset().y),
            Message::Input | Message::Edited => None,
        })
        .collect();

    assert!(
        offsets.last().is_some_and(|offset| *offset > 0.0),
        "the scrollable should have scrolled, got offsets: {offsets:?}"
    );
}

#[test]
fn text_editor_captures_wheel() {
    let content = text_editor::Content::with_text(&"line\n".repeat(100));

    let mut simulator = simulate(
        text_editor(&content)
            .height(100)
            .on_action(|_| Message::Edited),
    );
    simulator.point_at(Point::new(10.0, 10.0));

    assert_eq!(simulator.scroll(WHEEL_DOWN), event::Status::Captured);
}
