use std::{fmt::Display, fs};
use std::io::Write;


const MOD_X_FACTOR: f32 = 0.5;
const MOD_DOWN_FACTOR: f32 = 0.5;
const MOD_UP_FACTOR: f32 = 0.3;

#[derive(Debug, Clone, Copy)]
pub enum StickKind {
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
pub struct Stick {
    x: f32,
    y: f32,
}

pub struct Controller {
    pipe: fs::File,
    control_stick: Stick,
    c_stick: Stick,
    mod_x: bool,
    mod_y: bool,
}

impl Controller {
    pub fn new(pipe: fs::File) -> Controller {
        Controller {
            pipe,
            control_stick: Stick { x: 0.0, y: 0.0 },
            c_stick: Stick { x: 0.0, y: 0.0 },
            mod_x: false,
            mod_y: false,
        }
    }

    pub fn press_button(&mut self, button: &str) {
        let _ = self
            .pipe
            .write(format!("PRESS {}\n", button.to_string()).as_bytes());
    }

    pub fn release_button(&mut self, button: &str) {
        let _ = self
            .pipe
            .write(format!("RELEASE {}\n", button.to_string()).as_bytes());
    }

    pub fn press_trigger(&mut self, trigger: &str) {
        let _ = self
            .pipe
            .write(format!("PRESS {}\n", trigger.to_string()).as_bytes());
    }

    pub fn release_trigger(&mut self, trigger: &str) {
        let _ = self
            .pipe
            .write(format!("RELEASE {}\n", trigger.to_string()).as_bytes());
    }

    pub fn press_mod(&mut self, axis: &str) {
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

    pub fn release_mod(&mut self, axis: &str) {
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

    pub fn write_stick(&mut self, stick: StickKind, stick_data: Stick) {
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
                    if self.mod_y && self.control_stick.y < 0.0 {
                        stick_data.y * MOD_DOWN_FACTOR
                    } else if self.mod_y {
                        stick_data.y * MOD_UP_FACTOR
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
    pub fn tilt_stick(&mut self, stick: StickKind, direction: &str) {
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

    pub fn release_direction(&mut self, stick: StickKind, direction: &str) {
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
