use crate::controller::{Controller, StickKind};
use crate::keycodes;
use crate::keycodes::{get_keycode, Keymap};

pub struct KeyHandler {
    keys_held: Vec<String>,
    controller: Controller,
    keymap: Keymap,
}

impl KeyHandler {
    pub fn new(controller: Controller, keymap: Keymap) -> KeyHandler {
        KeyHandler {
            keys_held: Vec::new(),
            controller,
            keymap,
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
                self.controller.tilt_stick(StickKind::Control, dir);
            }
            None => {}
        }

        // see if we pressed the c stick
        match self.keymap.c_stick.get(&key_str) {
            Some(dir) => {
                self.controller.tilt_stick(StickKind::C, dir);
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
            Some(dir) => {
                self.controller.release_direction(StickKind::Control, dir);
            }
            None => {}
        }
        match self.keymap.c_stick.get(&key_str) {
            Some(dir) => {
                self.controller.release_direction(StickKind::C, dir);
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
