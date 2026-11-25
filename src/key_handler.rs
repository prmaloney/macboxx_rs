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
        if let Some(button) = self.keymap.buttons.get(&key_str) {
            self.controller.press_button(button);
        }

        // see if we pressed the control stick
        if let Some(dir) = self.keymap.control_stick.get(&key_str) {
            self.controller
                .tilt_stick(StickKind::Control, Direction::from_string(dir));
        }

        // see if we pressed the c stick
        if let Some(dir) = self.keymap.c_stick.get(&key_str) {
            self.controller
                .tilt_stick(StickKind::C, Direction::from_string(dir));
        }

        // see if we pressed a trigger
        if let Some(trigger) = self.keymap.triggers.get(&key_str) {
            self.controller.press_trigger(trigger);
        }

        // see if we pressed a modifier
        if let Some(modifier) = self.keymap.mods.get(&key_str) {
            self.controller.press_mod(modifier);
        }
    }

    pub fn on_release(&mut self, key: rdev::Key) {
        let key_str = keycodes::get_keycode(key);
        self.keys_held.retain(|k| k != &key_str);
        if let Some(button) = self.keymap.buttons.get(&key_str) {
            self.controller.release_button(button);
        }
        if let Some(dir_string) = self.keymap.control_stick.get(&key_str) {
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
        if let Some(dir) = self.keymap.c_stick.get(&key_str) {
            self.controller
                .release_direction(StickKind::C, Direction::from_string(dir));
        }
        if let Some(trigger) = self.keymap.triggers.get(&key_str) {
            self.controller.release_trigger(trigger);
        }
        if let Some(modifier) = self.keymap.mods.get(&key_str) {
            self.controller.release_mod(modifier);
        }
    }
}
