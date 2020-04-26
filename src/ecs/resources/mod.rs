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

#[derive(Default)]
pub struct GameField {
    pub field: Vec<Vec<char>>
}

impl GameField {
    pub fn new() -> Self {
        let field = vec!(
            Vec::from("############"),
            Vec::from("#          #"),
            Vec::from("#          #"),
            Vec::from("#          #"),
            Vec::from("#          #"),
            Vec::from("#          #"),
            Vec::from("#          #"),
            Vec::from("#          #"),
            Vec::from("#          #"),
            Vec::from("#          #"),
            Vec::from("#          #"),
            Vec::from("############"),
        );
        let field_char = field.iter()
            .map(|x| x.iter()
                .map(|y| *y as char).collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        GameField { field: field_char }
    }
}