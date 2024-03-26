use ini::Ini;
mod keycodes;
use keycodes::Stringable;
use libc::mkfifo;
use rdev::{listen, Event, EventType};
use serde_derive::Deserialize;
use std::{env, ffi::CString, fs, io::Write, path::Path};

fn create_pipe(slippi_path: &Path) -> fs::File {
    let pipe_dir = slippi_path.join("Pipes");
    let pipe_path = pipe_dir.join("macboxx");
    if pipe_path.exists() {
        return fs::File::create(&pipe_path).unwrap();
    }

    std::fs::create_dir_all(&pipe_dir).unwrap();
    let pipe_filename = CString::new(pipe_path.to_str().unwrap().as_bytes()).unwrap();
    unsafe {
        if mkfifo(pipe_filename.as_ptr(), 0444) != 0 {
            panic!("failed to make fifo");
        }
    }

    println!("writing to {:?}", &pipe_path);
    fs::File::create(&pipe_path).unwrap()
}

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

#[derive(Deserialize, Debug)]
struct Keymap {
    buttons: std::collections::HashMap<String, String>,
    control_stick: std::collections::HashMap<String, String>,
    c_stick: std::collections::HashMap<String, String>,
    triggers: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Copy)]
enum StickKind {
    Control,
    C,
}

#[derive(Debug, Clone, Copy)]
struct Stick {
    x: f32,
    y: f32,
}

struct Controller {
    pipe: fs::File,
    control_stick: Stick,
    c_stick: Stick,
}

impl Controller {
    fn new(pipe: fs::File) -> Controller {
        Controller {
            pipe,
            control_stick: Stick { x: 0.5, y: 0.5 },
            c_stick: Stick { x: 0.5, y: 0.5 },
        }
    }

    fn press_button(&mut self, button: &str) {
        let _ = self
            .pipe
            .write(format!("PRESS {}\n", button.to_string()).as_bytes());
    }

    fn release_button(&mut self, button: &str) {
        let _ = self
            .pipe
            .write(format!("RELEASE {}\n", button.to_string()).as_bytes());
    }

    // TODO: adjust all this logic
    fn tilt_stick(&mut self, stick: StickKind, direction: &str, mod_x: bool, mod_y: bool) {
        match stick {
            StickKind::Control => {
                self.control_stick = match direction {
                    "up" => Stick {
                        x: self.control_stick.x,
                        y: self.control_stick.y + 0.01,
                    },
                    "down" => Stick {
                        x: self.control_stick.x,
                        y: self.control_stick.y - 0.01,
                    },
                    "left" => Stick {
                        x: self.control_stick.x - 0.01,
                        y: self.control_stick.y,
                    },
                    "right" => Stick {
                        x: self.control_stick.x + 0.01,
                        y: self.control_stick.y,
                    },
                    _ => self.control_stick,
                }
            }
            StickKind::C => {
                self.c_stick = match direction {
                    "up" => Stick {
                        x: self.c_stick.x,
                        y: self.c_stick.y + 0.01,
                    },
                    "down" => Stick {
                        x: self.c_stick.x,
                        y: self.c_stick.y - 0.01,
                    },
                    "left" => Stick {
                        x: self.c_stick.x - 0.01,
                        y: self.c_stick.y,
                    },
                    "right" => Stick {
                        x: self.c_stick.x + 0.01,
                        y: self.c_stick.y,
                    },
                    _ => self.c_stick,
                }
            }
        }
    }

    fn release_direction(&mut self, stick: StickKind, direction: &str) {
        match stick {
            StickKind::Control => {
                self.control_stick = match direction {
                    "up" => Stick {
                        x: self.control_stick.x,
                        y: self.control_stick.y - 0.01,
                    },
                    "down" => Stick {
                        x: self.control_stick.x,
                        y: self.control_stick.y + 0.01,
                    },
                    "left" => Stick {
                        x: self.control_stick.x + 0.01,
                        y: self.control_stick.y,
                    },
                    "right" => Stick {
                        x: self.control_stick.x - 0.01,
                        y: self.control_stick.y,
                    },
                    _ => self.control_stick,
                }
            }

            StickKind::C => {
                self.control_stick = match direction {
                    "up" => Stick {
                        x: self.control_stick.x,
                        y: self.control_stick.y - 0.01,
                    },
                    "down" => Stick {
                        x: self.control_stick.x,
                        y: self.control_stick.y + 0.01,
                    },
                    "left" => Stick {
                        x: self.control_stick.x + 0.01,
                        y: self.control_stick.y,
                    },
                    "right" => Stick {
                        x: self.control_stick.x - 0.01,
                        y: self.control_stick.y,
                    },
                    _ => self.control_stick,
                }
            }
        }
    }
}

fn main() {
    let slippi_path = env::args().nth(1).expect("no slippi path provided");
    if !Path::new(&slippi_path).exists() {
        panic!("slippi path does not exist");
    }

    setup_config(&slippi_path);

    let pipe = create_pipe(&Path::new(&slippi_path).join("User"));

    let mut buttons_held: Vec<String> = vec![];

    let keymap_file_contents = match fs::read_to_string("keymap.toml") {
        Ok(contents) => contents,
        Err(_) => {
            eprintln!("keymap.toml not found. Please create one");
            std::process::exit(1);
        }
    };

    let keymap: Keymap = match toml::from_str::<Keymap>(&keymap_file_contents) {
        Ok(k) => Keymap {
            buttons: k
                .buttons
                .iter()
                .map(|(k, v)| (v.to_string(), k.to_string()))
                .collect(),
            control_stick: k
                .control_stick
                .iter()
                .map(|(k, v)| (v.to_string(), k.to_string()))
                .collect(),
            c_stick: k
                .c_stick
                .iter()
                .map(|(k, v)| (v.to_string(), k.to_string()))
                .collect(),
            triggers: k
                .triggers
                .iter()
                .map(|(k, v)| (v.to_string(), k.to_string()))
                .collect(),
        },
        Err(_) => {
            eprintln!("Could not parse keymap.toml");
            std::process::exit(1);
        }
    };
    println!("read keymap from file, {:?}", keymap);

    let mut controller = Controller::new(pipe);

    let on_key_press = move |event: Event| {
        match event.event_type {
            EventType::KeyPress(key) => {
                let key_str = key.as_string();
                // see if we pressed a button
                match keymap.buttons.get(&key_str) {
                    Some(button) => {
                        if buttons_held.contains(&button) {
                            return;
                        }
                        buttons_held.push(button.to_string());
                        controller.press_button(button);
                    }
                    None => {}
                }

                match keymap.control_stick.get(&key_str) {
                    Some(dir) => {
                        controller.tilt_stick(StickKind::Control, dir, false, false);
                    }
                    None => {}
                }

                match keymap.c_stick.get(&key_str) {
                    Some(dir) => {
                        controller.tilt_stick(StickKind::C, dir, false, false);
                    }
                    None => {}
                }
            }
            EventType::KeyRelease(key) => {
                let key_str = key.as_string();
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
            }
            _ => {}
        }
        println!("{:?}", buttons_held);
    };

    if let Err(error) = listen(on_key_press) {
        println!("Error: {:?}", error)
    }
}
