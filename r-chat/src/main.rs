use fltk::{
    prelude::*, app, window::Window, browser::Browser,
    button::Button, enums::Color, frame::Frame,
    text::{TextEditor, TextBuffer},
};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures::{StreamExt, SinkExt};

use std::process;
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::mpsc::UnboundedReceiver;

async fn websocket_task(tx: UnboundedSender<String>, mut disconnect_rx: UnboundedReceiver<()>) {
    let url = "ws://XX.XX.XX.XX:8080";

    match connect_async(url).await {
        Ok((mut ws_stream, _)) => {
            tx.send("Connected".to_string()).unwrap();
            let (mut write, _) = ws_stream.split();

            tokio::select! {
                _ = disconnect_rx.recv() => {
                    println!("WebSocket connection closing...");
                }
                else => {
                    write.send(Message::Text("Hello WebSocket".into())).await.unwrap();
                }
            }
        }
        Err(e) => {
            eprintln!("Connection error: {}", e);
            tx.send(format!("Connection error: {}", e)).unwrap();
        }
    }
}

fn main() {
    let app = app::App::default();
    let mut win = Window::new(100, 100, 800, 600, "Application");

    let mut message_display = Browser::new(400, 0, 400, 600, "");
    message_display.add("Welcome on the chat!");

    let mut separator = Frame::new(0, 300, 400, 2, "");
    separator.set_color(Color::Black);
    separator.set_frame(fltk::enums::FrameType::FlatBox);

    let mut connected_label = Frame::new(10, 35, 80, 15, "Total users: 0");
    connected_label.set_label_size(11);
    connected_label.set_pos(0, 20);
    connected_label.set_label_color(Color::Blue);

    let mut lower_label = Frame::new(3, 310, 80, 15, "Message");
    lower_label.set_label_size(11);
    lower_label.set_pos(0, 310);
    lower_label.set_label_color(Color::Black);

    let mut text_editor = TextEditor::new(10, 330, 380, 150, "");
    let text_buffer = TextBuffer::default();
    text_editor.set_buffer(Some(text_buffer));

    let mut exit_btn = Button::default()
        .with_size(50, 30)
        .with_label("Exit")
        .with_pos(15, 560);
    exit_btn.set_color(Color::Red);
    exit_btn.set_callback(move |_| {
        process::exit(0);
    });

    let mut send_btn = Button::default()
        .with_size(50, 30)
        .with_label("Send")
        .with_pos(85, 560);
    send_btn.set_color(Color::Green);
    send_btn.set_callback(move |_| {
        let text = text_editor.buffer().unwrap().text();
        println!("{}", text);
        text_editor.buffer().unwrap().set_text("");
    });


    let (tx, mut rx) = unbounded_channel::<String>();

    let mut connect_btn = Button::new(190, 560, 60, 30, "Connect");
    let mut disconnect_btn = Button::new(290, 560, 80, 30, "Disconnect");

    let rt = tokio::runtime::Runtime::new().unwrap();

    connect_btn.set_callback(move |_| {
        let tx = tx.clone();

        let (disconnect_tx, disconnect_rx) = unbounded_channel::<()>();

        rt.spawn(async move {
            websocket_task(tx, disconnect_rx).await;
        });

        disconnect_btn.set_callback(move |_| {
            disconnect_tx.send(()).unwrap();
        });
    });


    win.end();
    win.show();

    app::add_timeout3(5.0, move |_| {
        message_display.remove(1);
        message_display.add(" /\\_/\\  ");
        message_display.add("( o.o ) > \"Hi!\"");
        message_display.add(" > ^ < ");
    });

    app.run().unwrap();
}
