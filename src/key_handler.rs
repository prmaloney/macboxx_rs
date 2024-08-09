use crate::controller::{Controller, Direction, StickKind};
use crate::keycodes;
use crate::keycodes::{get_keycode, Keymap};

pub struct KeyHandler {
    keys_held: Vec<String>,
    controller: Controller,
    keymap: Keymap,
    control_stick_reverse: std::collections::HashMap<String, String>,
}

impl KeyHandler {
    pub fn new(controller: Controller, keymap: Keymap) -> KeyHandler {
        KeyHandler {
            keys_held: Vec::new(),
            controller,
            keymap: keymap.clone(),
            control_stick_reverse: keymap
                .control_stick
                .iter()
                .map(|(k, v)| (v.to_string(), k.to_string()))
                .collect(),
        }
    }

    pub fn on_press(&mut self, key: rdev::Key) {
        let key_str = get_keycode(key);
        if self.keys_held.contains(&key_str) {
            return;
        }
        self.keys_held.push(key_str.clone());

        // see if we pressed a button
        match self.keymap.buttons.get(&key_str) {
            Some(button) => {
                self.controller.press_button(button);
            }
            None => {}
        }

        // see if we pressed the control stick
        match self.keymap.control_stick.get(&key_str) {
            Some(dir) => {
                self.controller
                    .tilt_stick(StickKind::Control, Direction::from_string(dir));
            }
            None => {}
        }

        // see if we pressed the c stick
        match self.keymap.c_stick.get(&key_str) {
            Some(dir) => {
                self.controller
                    .tilt_stick(StickKind::C, Direction::from_string(dir));
            }
            None => {}
        }

        // see if we pressed a trigger
        match self.keymap.triggers.get(&key_str) {
            Some(trigger) => {
                self.controller.press_trigger(trigger);
            }
            None => {}
        }

        // see if we pressed a modifier
        match self.keymap.mods.get(&key_str) {
            Some(modifier) => {
                self.controller.press_mod(modifier);
            }
            None => {}
        }
    }

    pub fn on_release(&mut self, key: rdev::Key) {
        let key_str = keycodes::get_keycode(key);
        self.keys_held.retain(|k| k != &key_str);
        match self.keymap.buttons.get(&key_str) {
            Some(button) => {
                self.controller.release_button(button);
            }
            None => {}
        }
        match self.keymap.control_stick.get(&key_str) {
            Some(dir_string) => {
                let dir: Direction = Direction::from_string(dir_string);
                if let Some(opposite_key) = self.control_stick_reverse.get(&dir.opposite().as_string()) {
                    if self.keys_held.contains(opposite_key) {
                        self.controller
                            .tilt_stick(StickKind::Control, dir.opposite())
                    } else {
                        self.controller.release_direction(StickKind::Control, dir);
                    }
                }
            }
            None => {}
        }
        match self.keymap.c_stick.get(&key_str) {
            Some(dir) => {
                self.controller
                    .release_direction(StickKind::C, Direction::from_string(dir));
            }
            None => {}
        }
        match self.keymap.triggers.get(&key_str) {
            Some(trigger) => {
                self.controller.release_trigger(trigger);
            }
            None => {}
        }
        match self.keymap.mods.get(&key_str) {
            Some(modifier) => {
                self.controller.release_mod(modifier);
            }
            None => {}
        }
    }
}
