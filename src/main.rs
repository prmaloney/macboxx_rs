use crate::controller::Controller;
use clap::Parser;
use keycodes::Keymap;
use rdev::{listen, Event, EventType};
use std::path::Path;

mod config;
mod controller;
mod keycodes;
mod pipe;
mod key_handler;


#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    // Path to slippi 'netplay' directory
    slippi: String,

    // Location of your keymap.toml file
    #[arg(short, long)]
    keymap: String,
}

fn main() {
    let args = Args::parse();
    let slippi_path = args.slippi;

    config::setup_config(&slippi_path);

    let keymap: Keymap = keycodes::setup_keymap(args.keymap).expect("Failed to load keymap");

    let pipe = pipe::create_pipe(&Path::new(&slippi_path).join("User"));
    let controller = Controller::new(pipe);

    let mut handler = key_handler::KeyHandler::new(controller, keymap);

    let on_event = move |event: Event| match event.event_type {
        EventType::KeyPress(key) => handler.on_press(key),

        EventType::KeyRelease(key) => handler.on_release(key),
        _ => {}
    };

    if let Err(error) = listen(on_event) {
        println!("Error: {:?}", error)
    }
}
