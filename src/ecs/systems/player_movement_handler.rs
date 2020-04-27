use std::time::Duration;

use specs::prelude::*;

use crate::ecs::components::Player;
use crate::ecs::components::position::Position;
use crate::ecs::resources::{Clock, Counter, GameField, GameStatus};
use crate::ecs::resources::event::{Event, Events};
use crate::ecs::resources::event::Event::MoveUp;

pub struct PMoveHandler;

#[derive(SystemData)]
pub struct PMHData<'a> {
    pub events: Write<'a, Events>,
    pub player: ReadStorage<'a, Player>,
    pub pos: WriteStorage<'a, Position>,
    pub clock: Read<'a, Clock>,
    pub field: Read<'a, GameField>,
}

impl<'a> System<'a> for PMoveHandler {
    type SystemData = PMHData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let mut pos = data.pos;
        let mut movement: (f32, f32) = (0.0, 0.0);
        let mut events = data.events;

        match events.last_event {
            Some(Event::MoveUp) => {
                movement.0 += 1.0;
                events.last_event = None;
            }
            Some(Event::MoveDown) => {
                movement.0 -= 1.0;
                events.last_event = None;
            }
            Some(Event::MoveLeft) => {
                movement.1 -= 0.5;
                events.last_event = None;
            }
            Some(Event::MoveRight) => {
                movement.1 += 0.5;
                events.last_event = None;
            }
            _ => {}
        }

        for (_, pos) in (&data.player, &mut pos).join() {
            pos.x += data.clock.unbind(movement.0 * pos.angle.cos());
            pos.y += data.clock.unbind(movement.0 * pos.angle.sin());
            if data.field.field[pos.x as usize][pos.y as usize] == '#' {
                pos.x -= data.clock.unbind(movement.0 * pos.angle.cos());
                pos.y -= data.clock.unbind(movement.0 * pos.angle.sin());
            }
            pos.angle += data.clock.unbind(movement.1);
        }
    }
}
