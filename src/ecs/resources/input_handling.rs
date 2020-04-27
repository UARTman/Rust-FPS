use rustbox::Key;

pub struct KeyPresses(pub Vec<Key>);


impl Default for KeyPresses {
    fn default() -> Self {
        KeyPresses(Vec::new())
    }
}