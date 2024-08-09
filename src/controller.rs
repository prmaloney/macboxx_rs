use std::{fmt::Display, fs};
use std::io::Write;

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
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "UP"),
            Direction::Down => write!(f, "DOWN"),
            Direction::Left => write!(f, "LEFT"),
            Direction::Right => write!(f, "RIGHT"),
        }
    }
}

impl Direction {
    pub fn as_string(&self) -> String {
        match self {
            Direction::Up => "UP".to_string(),
            Direction::Down => "DOWN".to_string(),
            Direction::Left => "LEFT".to_string(),
            Direction::Right => "RIGHT".to_string(),
        }
    }
}

impl Direction {
    pub fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn from_string(s: &str) -> Direction {
        match s {
            "UP" => Direction::Up,
            "DOWN" => Direction::Down,
            "LEFT" => Direction::Left,
            "RIGHT" => Direction::Right,
            _ => panic!("Unknown direction: {}", s),
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
    mod_x_factor: f32,
    mod_up_factor: f32,
    mod_down_factor: f32,
}

impl Controller {
    pub fn new(pipe: fs::File, mod_x_factor: f32, mod_up_factor: f32, mod_down_factor: f32) -> Controller {
        Controller {
            pipe,
            control_stick: Stick { x: 0.0, y: 0.0 },
            c_stick: Stick { x: 0.0, y: 0.0 },
            mod_x: false,
            mod_y: false,
            mod_x_factor,
            mod_up_factor,
            mod_down_factor,
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
                        stick_data.x * self.mod_x_factor
                    } else {
                        stick_data.x
                    }
                });
                let y = Self::convert_coords({
                    if self.mod_y && self.control_stick.y < 0.0 {
                        stick_data.y * self.mod_down_factor
                    } else if self.mod_y {
                        stick_data.y * self.mod_up_factor
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

    pub fn tilt_stick(&mut self, stick: StickKind, direction: Direction) {
        match stick {
            StickKind::Control => {
                self.control_stick = match direction {
                    Direction::Up => Stick {
                        y: 1.0,
                        x: self.control_stick.x,
                    },
                    Direction::Down => Stick {
                        y: -1.0,
                        x: self.control_stick.x,
                    },
                    Direction::Left => Stick {
                        x: -1.0,
                        y: self.control_stick.y,
                    },
                    Direction::Right => Stick {
                        x: 1.0,
                        y: self.control_stick.y,
                    },
                };
                self.write_stick(StickKind::Control, self.control_stick);
            }
            StickKind::C => {
                self.c_stick = match direction {
                    Direction::Up => Stick {
                        x: self.c_stick.x,
                        y: 1.0,
                    },
                    Direction::Down => Stick {
                        x: self.c_stick.x,
                        y: -1.0,
                    },
                    Direction::Left => Stick {
                        x: -1.0,
                        y: self.c_stick.y,
                    },
                    Direction::Right => Stick {
                        x: 1.0,
                        y: self.c_stick.y,
                    },
                };
                self.write_stick(StickKind::C, self.c_stick);
            }
        }
    }

    pub fn release_direction(&mut self, stick: StickKind, direction: Direction) {
        match stick {
            StickKind::Control => {
                self.control_stick = match direction {
                    Direction::Up => Stick {
                        x: self.control_stick.x,
                        y: 0.0,
                    },
                    Direction::Down => Stick {
                        x: self.control_stick.x,
                        y: 0.0,
                    },
                    Direction::Left => Stick {
                        x: if self.control_stick.x > 0.0 {
                            self.control_stick.x
                        } else {
                            0.0
                        },
                        y: self.control_stick.y,
                    },
                    Direction::Right => Stick {
                        x: if self.control_stick.x < 0.0 {
                            self.control_stick.x
                        } else {
                            0.0
                        },
                        y: self.control_stick.y,
                    },
                };
                self.write_stick(StickKind::Control, self.control_stick);
            }

            StickKind::C => {
                self.c_stick = match direction {
                    Direction::Up => Stick {
                        x: self.c_stick.x,
                        y: 0.0,
                    },
                    Direction::Down => Stick {
                        x: self.c_stick.x,
                        y: 0.0,
                    },
                    Direction::Left => Stick {
                        x: 0.0,
                        y: self.c_stick.y,
                    },
                    Direction::Right => Stick {
                        x: 0.0,
                        y: self.c_stick.y,
                    },
                };
                self.write_stick(StickKind::C, self.c_stick);
            }
        }
    }
}
