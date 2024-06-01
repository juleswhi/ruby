#![allow(dead_code)]

use std::io::prelude::*;
use std::net::TcpStream;

use gtk4::{prelude::*, ApplicationWindow, Button};
use gtk4::{glib, Application};

const APP_ID: &str = "org.juleswhi.ruby";

const SERVER_ADDY: &str = "127.0.0.1:2409";

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

    button.connect_clicked(|_| {
        let _ = send_req();
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Ruby")
        .child(&button)
        .build();

    window.present();
}

fn send_req() -> std::io::Result<()>{
    let mut stream = TcpStream::connect(SERVER_ADDY)?;

    stream.write(&[1])?;
    // stream.read(&mut [0; 128])?;

    Ok(())
}

