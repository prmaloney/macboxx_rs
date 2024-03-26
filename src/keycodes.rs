use rdev::Key;

pub(crate) trait Stringable {
    fn as_string(&self) -> String;
}

impl Stringable for Key {
    fn as_string(&self) -> String {
        format!("{:?}", self).replace("Key", "").to_lowercase()
    }
}
