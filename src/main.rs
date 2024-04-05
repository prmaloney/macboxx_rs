use crate::controller::Controller;
use crate::controller::StickKind;
use clap::Parser;
use ini::Ini;
use keycodes::Keymap;
use keycodes::Stringable;
use rdev::{listen, Event, EventType};
use std::path::Path;

mod controller;
mod keycodes;
mod pipe;

fn setup_config(slippi_path: &String) {
    let config_path = Path::new(&slippi_path).join("User").join("Config");
    let mut gc_config = Ini::load_from_file(config_path.join("GCPadNew.ini")).unwrap();
    let port_1_section = gc_config.section(Some("GCPad1")).unwrap();
    if port_1_section.get("Device").unwrap() == "Pipe/0/macboxx" {
        println!("Controller already configured")
    } else {
        gc_config
            .with_section(Some("GCPad1"))
            .set("Device", "Pipe/0/macboxx")
            .set("Buttons/A", "Button A")
            .set("Buttons/B", "Button B")
            .set("Buttons/X", "Button X")
            .set("Buttons/Y", "Button Y")
            .set("Buttons/Z", "Button Z")
            .set("Buttons/L", "Button L")
            .set("Buttons/R", "Button R")
            .set("Buttons/Threshold", "50.00000000000000")
            .set("Main Stick/Up", "Axis MAIN Y +")
            .set("Main Stick/Down", "Axis MAIN Y -")
            .set("Main Stick/Left", "Axis MAIN X -")
            .set("Main Stick/Right", "Axis MAIN X +")
            .set("Triggers/L", "Button L")
            .set("Triggers/R", "Button R")
            .set("Main Stick/Modifier", "Shift_L")
            .set("Main Stick/Modifier/Range", "50.000000000000000")
            .set("Main Stick/Radius", "100.000000000000000")
            .set("D-Pad/Up", "Button D_UP")
            .set("D-Pad/Down", "Button D_DOWN")
            .set("D-Pad/Left", "Button D_LEFT")
            .set("D-Pad/Right", "Button D_RIGHT")
            .set("Buttons/Start", "Button START")
            .set("Buttons/A", "Button A")
            .set("C-Stick/Up", "Axis C Y +")
            .set("C-Stick/Down", "Axis C Y -")
            .set("C-Stick/Left", "Axis C X -")
            .set("C-Stick/Right", "Axis C X +")
            .set("C-Stick/Radius", "100.000000000000000")
            .set("Triggers/L-Analog", "Axis L -+")
            .set("Triggers/R-Analog", "Axis R -+")
            .set("Triggers/Threshold", "90.00000000000000");

        gc_config
            .write_to_file(config_path.join("GCPadNew.ini"))
            .unwrap();
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
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
    let keymap: Keymap = keycodes::setup_keymap(args.keymap).expect("Failed to load keymap");

    setup_config(&slippi_path);

    let pipe = pipe::create_pipe(&Path::new(&slippi_path).join("User"));
    let mut controller = Controller::new(pipe);


    let mut keys_held: Vec<String> = vec![];

    let on_key_press = move |event: Event| {
        match event.event_type {
            EventType::KeyPress(key) => {
                let key_str = keycodes::get_keycode(key);
                if keys_held.contains(&key_str) {
                    return;
                }
                keys_held.push(key_str.clone());

                // see if we pressed a button
                match keymap.buttons.get(&key_str) {
                    Some(button) => {
                        controller.press_button(button);
                    }
                    None => {}
                }

                // see if we pressed the control stick
                match keymap.control_stick.get(&key_str) {
                    Some(dir) => {
                        controller.tilt_stick(StickKind::Control, dir);
                    }
                    None => {}
                }

                // see if we pressed the c stick
                match keymap.c_stick.get(&key_str) {
                    Some(dir) => {
                        controller.tilt_stick(StickKind::C, dir);
                    }
                    None => {}
                }

                // see if we pressed a trigger
                match keymap.triggers.get(&key_str) {
                    Some(trigger) => {
                        controller.press_trigger(trigger);
                    }
                    None => {}
                }

                // see if we pressed a modifier
                match keymap.mods.get(&key_str) {
                    Some(modifier) => {
                        controller.press_mod(modifier);
                    }
                    None => {}
                }
            }

            EventType::KeyRelease(key) => {
                let key_str = key.as_string();
                keys_held.retain(|k| k != &key_str);
                match keymap.buttons.get(&key_str) {
                    Some(button) => {
                        controller.release_button(button);
                    }
                    None => {}
                }
                match keymap.control_stick.get(&key_str) {
                    Some(dir) => {
                        controller.release_direction(StickKind::Control, dir);
                    }
                    None => {}
                }
                match keymap.c_stick.get(&key_str) {
                    Some(dir) => {
                        controller.release_direction(StickKind::C, dir);
                    }
                    None => {}
                }
                match keymap.triggers.get(&key_str) {
                    Some(trigger) => {
                        controller.release_trigger(trigger);
                    }
                    None => {}
                }
                match keymap.mods.get(&key_str) {
                    Some(modifier) => {
                        controller.release_mod(modifier);
                    }
                    None => {}
                }
            }
            _ => {}
        }
    };

    if let Err(error) = listen(on_key_press) {
        println!("Error: {:?}", error)
    }
}
