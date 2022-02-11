// https://gtk-rs.org/gtk4-rs/stable/latest/book/hello_world.html

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() {
    let app = Application::builder()
        .application_id("org.energonsoftware.hello")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(move |button| {
        button.set_label("Hello World!");
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hello World!")
        .child(&button)
        .build();

    window.present();
}
