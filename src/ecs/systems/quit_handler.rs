use rustbox::Key;
use specs::prelude::*;

use crate::ecs::resources::{GameStatus, KeyPresses, RustBoxWrapper, Counter, Clock};
use std::time::Duration;
use crate::ecs::resources::event::{Event, Events};

pub struct QuitHandler;

#[derive(SystemData)]
pub struct QHData<'a> {
    pub game_status: Write<'a, GameStatus>,
    pub events: Write<'a, Events>
}

impl<'a> System<'a> for QuitHandler {
    type SystemData = QHData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let mut gs = data.game_status;
        let mut events = data.events;
        // let mut for_deletion = Vec::new();
        match events.last_event {
            Some(Event::Quit) => {
                gs.0 = false;
                events.last_event = None;
            }
            _ => {}
        }
        // events.handled(for_deletion);

    }
}
