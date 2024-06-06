#![allow(dead_code)]

mod saph;

use std::io::prelude::*;
use std::net::TcpStream;

use gtk4::{glib, Application};
use gtk4::{prelude::*, ApplicationWindow, Button};

const APP_ID: &str = "org.juleswhi.ruby";

const SERVER_ADDY: &str = "127.0.0.1:2409";

fn main() -> glib::ExitCode {
    let str_res = TcpStream::connect(SERVER_ADDY);

    if let Ok(mut stream) = str_res {
        let mut req = saph::request::SaphRequest::new
            (SERVER_ADDY.to_owned());
        req.path = "/data".into();
        req.request_type = saph::RequestType::GET;
        println!("{}", req.request_type.string());

        let _ = stream.write_all(&req.to_bytes());
    }

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

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Ruby")
        .child(&button)
        .build();

    window.present();
}
