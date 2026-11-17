//! Tests for selectable `text`.
use iced_test::Simulator;
use iced_widget::core::keyboard::{self, Modifiers};
use iced_widget::core::mouse;
use iced_widget::core::widget::operation::{self, Operation};
use iced_widget::core::{Event, Never, Point, Theme, Widget};
use iced_widget::{Renderer, column, text};

fn view() -> impl Widget<Never, Theme, Renderer> + 'static {
    column![
        text("First line").selectable(true),
        text("Second line").selectable(true),
    ]
}

fn click_at(simulator: &mut Simulator<'_, Never, Theme, Renderer>, position: Point) {
    simulator.point_at(position);

    // Press and release in separate frames, so the selection is applied
    // while the button is held
    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);
    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);
}

fn press_modifiers(simulator: &mut Simulator<'_, Never, Theme, Renderer>, modifiers: Modifiers) {
    let _ = simulator.simulate([Event::Keyboard(keyboard::Event::ModifiersChanged(
        modifiers,
    ))]);
}

fn selection(simulator: &mut Simulator<'_, Never, Theme, Renderer>) -> String {
    let mut copy = operation::text::copy();
    simulator.operate(&mut operation::black_box(&mut copy));

    match copy.finish() {
        operation::Outcome::Some(text) => text,
        _ => String::new(),
    }
}

#[test]
fn shift_click_selects_from_last_click() {
    let mut simulator = Simulator::new(view());
    let start = Point::new(1.0, 10.0);
    let end = Point::new(1000.0, 30.0);

    click_at(&mut simulator, start);
    assert_eq!(selection(&mut simulator), "");

    press_modifiers(&mut simulator, Modifiers::SHIFT);
    click_at(&mut simulator, end);
    assert_eq!(selection(&mut simulator), "First line\nSecond line");

    press_modifiers(&mut simulator, Modifiers::empty());
    click_at(&mut simulator, end);
    assert_eq!(selection(&mut simulator), "");
}

#[test]
fn shift_click_keeps_anchor_across_extensions() {
    let mut simulator = Simulator::new(view());

    click_at(&mut simulator, Point::new(1.0, 10.0));

    press_modifiers(&mut simulator, Modifiers::SHIFT);
    click_at(&mut simulator, Point::new(1000.0, 30.0));
    click_at(&mut simulator, Point::new(1000.0, 10.0));

    assert_eq!(selection(&mut simulator), "First line");
}

#[test]
fn shift_click_after_double_click_extends_by_word() {
    let mut simulator = Simulator::new(view());
    let start = Point::new(1.0, 10.0);

    click_at(&mut simulator, start);
    click_at(&mut simulator, start);

    press_modifiers(&mut simulator, Modifiers::SHIFT);
    click_at(&mut simulator, Point::new(20.0, 30.0));

    assert_eq!(selection(&mut simulator), "First line\nSecond");
}

#[test]
fn shift_state_follows_event_order_within_a_batch() {
    let mut simulator = Simulator::new(view());

    click_at(&mut simulator, Point::new(1.0, 10.0));

    // Shift is released in the same batch as the press, after it, so the
    // press must still count as a shift+click
    simulator.point_at(Point::new(1000.0, 30.0));
    let _ = simulator.simulate([
        Event::Keyboard(keyboard::Event::ModifiersChanged(Modifiers::SHIFT)),
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)),
        Event::Keyboard(keyboard::Event::ModifiersChanged(Modifiers::empty())),
    ]);
    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);

    assert_eq!(selection(&mut simulator), "First line\nSecond line");
}
