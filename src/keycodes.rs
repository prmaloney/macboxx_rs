use std::fs;

use rdev::Key;
use serde_derive::Deserialize;

pub(crate) trait Stringable {
    fn as_string(&self) -> String;
}

impl Stringable for Key {
    fn as_string(&self) -> String {
        format!("{:?}", self).replace("Key", "")
    }
}

pub fn get_keycode(key: Key) -> String {
    match key {
        Key::Space => " ".to_string(),
        Key::BackQuote => "`".to_string(),
        Key::Minus => "-".to_string(),
        Key::Equal => "=".to_string(),
        Key::LeftBracket => "[".to_string(),
        Key::RightBracket => "]".to_string(),
        Key::SemiColon => ";".to_string(),
        Key::Quote => "'".to_string(),
        Key::BackSlash => "\\".to_string(),
        Key::IntlBackslash => "\\".to_string(),
        Key::Comma => ",".to_string(),
        Key::Dot => ".".to_string(),
        Key::Slash => "/".to_string(),
        _ => key.as_string(),
    }
}

#[derive(Deserialize, Debug)]
pub struct Keymap {
    pub buttons: std::collections::HashMap<String, String>,
    pub control_stick: std::collections::HashMap<String, String>,
    pub c_stick: std::collections::HashMap<String, String>,
    pub triggers: std::collections::HashMap<String, String>,
    pub mods: std::collections::HashMap<String, String>,
}

pub fn setup_keymap(keymap_path: String) -> Result<Keymap, String> {
    let keymap_file_contents = match fs::read_to_string(keymap_path) {
        Ok(contents) => contents,
        Err(_) => return Err(String::from("keymap.toml not found. Please create one")),
    };

    match toml::from_str::<Keymap>(&keymap_file_contents) {
        Ok(k) => Ok(Keymap {
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
        }),
        Err(_) => Err(String::from("Could not parse keymap.toml")),
    }
}
