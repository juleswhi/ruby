use gtk4::{prelude::*, ApplicationWindow, Button};
use gtk4::{glib, Application};

const APP_ID: &str = "org.juleswhi.ruby";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {

    let button = Button::builder()
        .label("Request Server")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(|button| {
        button.set_label("Hello, World!");
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Ruby")
        .child(&button)
        .build();

    window.present();
}
