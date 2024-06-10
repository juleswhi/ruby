#![allow(dead_code)]

mod saph;

use std::io::prelude::*;
use std::net::TcpStream;

use gtk4::{glib, Application, Entry};
use gtk4::{prelude::*, ApplicationWindow};

const APP_ID: &str = "org.juleswhi.ruby";
const SERVER_ADDY: &str = "127.0.0.1:2409";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let addr_bar = Entry::builder().placeholder_text("Enter URL").build();

    addr_bar.connect_activate(move |entry| {
        let text = entry.text();
        send_req(text.to_string());
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Ruby")
        .child(&addr_bar)
        .build();

    window.present();
}

fn send_req(content: String) {
    let str_res = TcpStream::connect(SERVER_ADDY);

    if let Ok(mut stream) = str_res {
        let mut req = saph::request::SaphRequest::new(SERVER_ADDY.to_owned());
        req.path = "/data".into();
        req.request_type = saph::RequestType::GET;
        req.content = content;
        println!("{}", req.request_type.string());

        let _ = stream.write_all(&req.to_bytes());
        let res = saph::response::SaphResponse::from(&mut stream);

        if let None = res {
            println!("Error parsing bytes");
        }

        let mut buf: Vec<u8> = vec![];
        let response = stream.read_to_end(&mut buf);

        match response {
            Ok(_) => println!("ReceivedData"),
            Err(_) => println!("ErrorReceiving"),
        }
    }
}
