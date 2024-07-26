mod widgets;

use widgets::Widget;

fn main() {
    let mut window = widgets::Window::new("Rust GUI Demo 1.23");

    window.add_widget(Box::new(widgets::Label::new(
        "This is a small text GUI demo.",
    )));
    window.add_widget(Box::new(widgets::Button::new("Click me!")));
    window.draw();
}
