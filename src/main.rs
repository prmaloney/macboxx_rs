use ini::Ini;
use libc::mkfifo;
use rdev::{listen, Event, EventType};
use std::{env, ffi::CString, fs, io::Write, path::Path};

// enum Buttons {
//     A,
//     B,
//     X,
//     Y,
//     Z,
//     L,
//     R,
//     MainUp,
//     MainDown,
//     MainLeft,
//     MainRight,
//     CUp,
//     CDown,
//     CLeft,
//     CRight,
//     Start,
//     DUp,
//     DDown,
//     DLeft,
//     DRight,
//     LAnalog,
//     RAnalog,
// }
//

fn create_pipe(slippi_path: &Path) -> fs::File {
    let pipe_dir = slippi_path.join("Pipes");
    let pipe_path = pipe_dir.join("macboxx");
    let pipe_filename = CString::new(pipe_path.to_str().unwrap().as_bytes()).unwrap();
    unsafe {
        mkfifo(pipe_filename.as_ptr(), 0o777);
    }

    println!("writing to {:?}", &pipe_path);
    fs::File::create(&pipe_path).unwrap()
}

fn main() {
    let slippi_path = env::args().nth(1).expect("no slippi path provided");
    if !Path::new(&slippi_path).exists() {
        panic!("slippi path does not exist");
    }

    let config_path = Path::new(&slippi_path).join("User").join("Config");

    let mut pipe_file = create_pipe(&Path::new(&slippi_path).join("User"));

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

    let mut keys_down: Vec<rdev::Key> = vec![];

    let callback = move |event: Event| {
        println!("event");
        match event.event_type {
            EventType::KeyPress(key) => {
                if keys_down.contains(&key) {
                    return;
                }
                println!("key pressed: {:?}", key);
                keys_down.push(key);
                pipe_file.write("PRESS A\n".as_bytes()).expect("failed to write to pipe");
            }
            EventType::KeyRelease(key) => {
                keys_down.retain(|&k| k != key);
            }
            _ => {}
        }
        println!("{:?}", keys_down);
    };

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}
