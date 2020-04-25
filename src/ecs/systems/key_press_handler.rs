use rustbox::Key;
use specs::prelude::*;

use crate::ecs::resources::{GameStatus, KeyPresses};

pub struct KeyPressHandler;

#[derive(SystemData)]
pub struct KPHData<'a> {
    pub keys: Write<'a, KeyPresses>,
    pub game_status: Write<'a, GameStatus>,
}

impl<'a> System<'a> for KeyPressHandler {
    type SystemData = KPHData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let mut keys = data.keys;
        let mut gs = data.game_status;
        while keys.0.len() != 0 {
            let key = keys.0.remove(0);
            match key {
                Key::Char('q') => gs.0 = false,
                _ => {}
            }
        }
    }
}
