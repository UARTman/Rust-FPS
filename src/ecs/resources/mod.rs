use std::rc::Rc;
use std::sync::{Arc, Mutex};

use rustbox::{Key, RustBox};

use crate::util::init_rustbox;

pub struct RustBoxWrapper(pub RustBox);

impl Default for RustBoxWrapper {
    fn default() -> Self {
        RustBoxWrapper(init_rustbox())
    }
}

pub struct GameStatus(pub bool);

impl Default for GameStatus {
    fn default() -> Self {
        GameStatus(true)
    }
}

pub struct KeyPresses(pub Vec<Key>);

impl Default for KeyPresses {
    fn default() -> Self {
        KeyPresses(Vec::new())
    }
}
