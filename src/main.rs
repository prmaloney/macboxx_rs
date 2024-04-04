use ini::Ini;
mod keycodes;
use keycodes::Stringable;
use libc::mkfifo;
use rdev::{listen, Event, EventType};
use serde_derive::Deserialize;
use std::{env, ffi::CString, fmt::Display, fs, io::Write, path::Path};

const MOD_X_FACTOR: f32 = 0.5;
const MOD_Y_FACTOR: f32 = 0.5;

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
    mods: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Copy)]
enum StickKind {
    Control,
    C,
}

impl Display for StickKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StickKind::Control => write!(f, "MAIN"),
            StickKind::C => write!(f, "C"),
        }
    }
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
    mod_x: bool,
    mod_y: bool,
}

impl Controller {
    fn new(pipe: fs::File) -> Controller {
        Controller {
            pipe,
            control_stick: Stick { x: 0.0, y: 0.0 },
            c_stick: Stick { x: 0.0, y: 0.0 },
            mod_x: false,
            mod_y: false,
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

    fn press_trigger(&mut self, trigger: &str) {
        let _ = self
            .pipe
            .write(format!("PRESS {}\n", trigger.to_string()).as_bytes());
    }

    fn release_trigger(&mut self, trigger: &str) {
        let _ = self
            .pipe
            .write(format!("RELEASE {}\n", trigger.to_string()).as_bytes());
    }

    fn press_mod(&mut self, axis: &str) {
        match axis {
            "MOD_X" => {
                self.mod_x = true;
            }
            "MOD_Y" => {
                self.mod_y = true;
            }
            _ => {
                println!("Unknown axis: {}", axis)
            }
        }
        self.write_stick(StickKind::Control, self.control_stick);
    }

    fn release_mod(&mut self, axis: &str) {
        match axis {
            "MOD_X" => {
                self.mod_x = false;
            }
            "MOD_Y" => {
                self.mod_y = false;
            }
            _ => {
                println!("Unknown axis: {}", axis)
            }
        };
        self.write_stick(StickKind::Control, self.control_stick);
    }

    fn convert_coords(coord: f32) -> f32 {
        (coord + 1.0) / 2.0
    }

    fn write_stick(&mut self, stick: StickKind, stick_data: Stick) {
        match stick {
            StickKind::Control => {
                let x = Self::convert_coords({
                    if self.mod_x {
                        stick_data.x * MOD_X_FACTOR
                    } else {
                        stick_data.x
                    }
                });
                let y = Self::convert_coords({
                    if self.mod_y {
                        stick_data.y * MOD_Y_FACTOR
                    } else {
                        stick_data.y
                    }
                });
                self.pipe
                    .write(format!("SET {} {} {}\n", stick, x, y).as_bytes())
                    .unwrap();
            }
            StickKind::C => {
                self.pipe
                    .write(
                        format!(
                            "SET {} {} {}\n",
                            stick,
                            Self::convert_coords(stick_data.x),
                            Self::convert_coords(stick_data.y)
                        )
                        .as_bytes(),
                    )
                    .unwrap();
            }
        }
    }

    // TODO: adjust all this logic
    fn tilt_stick(&mut self, stick: StickKind, direction: &str) {
        match stick {
            StickKind::Control => {
                self.control_stick = match direction {
                    "UP" => Stick {
                        y: 1.0,
                        x: self.control_stick.x,
                    },
                    "DOWN" => Stick {
                        y: -1.0,
                        x: self.control_stick.x,
                    },
                    "LEFT" => Stick {
                        x: -1.0,
                        y: self.control_stick.y,
                    },
                    "RIGHT" => Stick {
                        x: 1.0,
                        y: self.control_stick.y,
                    },
                    _ => self.control_stick,
                };
                self.write_stick(StickKind::Control, self.control_stick);
            }
            StickKind::C => {
                self.c_stick = match direction {
                    "UP" => Stick {
                        x: self.c_stick.x,
                        y: 1.0,
                    },
                    "DOWN" => Stick {
                        x: self.c_stick.x,
                        y: -1.0,
                    },
                    "LEFT" => Stick {
                        x: -1.0,
                        y: self.c_stick.y,
                    },
                    "RIGHT" => Stick {
                        x: 1.0,
                        y: self.c_stick.y,
                    },
                    _ => self.c_stick,
                };
                self.write_stick(StickKind::C, self.c_stick);
            }
        }
    }

    fn release_direction(&mut self, stick: StickKind, direction: &str) {
        match stick {
            StickKind::Control => {
                self.control_stick = match direction {
                    "UP" => Stick {
                        x: self.control_stick.x,
                        y: 0.0,
                    },
                    "DOWN" => Stick {
                        x: self.control_stick.x,
                        y: 0.0,
                    },
                    "LEFT" => Stick {
                        x: if self.control_stick.x > 0.0 {
                            self.control_stick.x
                        } else {
                            0.0
                        },
                        y: self.control_stick.y,
                    },
                    "RIGHT" => Stick {
                        x: if self.control_stick.x < 0.0 {
                            self.control_stick.x
                        } else {
                            0.0
                        },
                        y: self.control_stick.y,
                    },
                    _ => self.control_stick,
                };
                self.write_stick(StickKind::Control, self.control_stick);
            }

            StickKind::C => {
                self.c_stick = match direction {
                    "UP" => Stick {
                        x: self.c_stick.x,
                        y: 0.0,
                    },
                    "DOWN" => Stick {
                        x: self.c_stick.x,
                        y: 0.0,
                    },
                    "LEFT" => Stick {
                        x: 0.0,
                        y: self.c_stick.y,
                    },
                    "RIGHT" => Stick {
                        x: 0.0,
                        y: self.c_stick.y,
                    },
                    _ => self.c_stick,
                };
                self.write_stick(StickKind::C, self.c_stick);
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

    let mut keys_held: Vec<String> = vec![];

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
            mods: k
                .mods
                .iter()
                .map(|(k, v)| (v.to_string(), k.to_string()))
                .collect(),
        },
        Err(_) => {
            eprintln!("Could not parse keymap.toml");
            std::process::exit(1);
        }
    };

    let mut controller = Controller::new(pipe);

    let on_key_press = move |event: Event| {
        match event.event_type {
            EventType::KeyPress(key) => {
                let key_str = key.as_string();
                if keys_held.contains(&key_str) {
                    return;
                }
                // see if we pressed a button
                keys_held.push(key_str.clone());
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
