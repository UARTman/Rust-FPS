use std::time::Duration;

use rustbox::{Key, RB_NORMAL};
use rustbox::Color::{Black, White};
use specs::prelude::*;

use crate::ecs::components::Player;
use crate::ecs::components::position::Position;
use crate::ecs::resources::{Clock, Counter, GameField, GameStatus, KeyPresses, RustBoxWrapper};
use crate::ecs::resources::event::{Event, Events};
use crate::ecs::resources::event::Event::MoveUp;

pub struct PMoveHandler;

#[derive(SystemData)]
pub struct PMHData<'a> {
    pub rustbox: Read<'a, RustBoxWrapper>,
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
                movement.1 -= 0.3;
                events.last_event = None;
            }
            Some(Event::MoveRight) => {
                movement.1 += 0.3;
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

            // data.rustbox.0.print(2, 2, RB_NORMAL, White, Black, format!("X:{}", pos.x).as_str());
            // data.rustbox.0.print(2, 3, RB_NORMAL, White, Black, format!("Y:{}", pos.y).as_str());
            // data.rustbox.0.print(2, 4, RB_NORMAL, White, Black, format!("Angle in rad:{}", pos.angle).as_str());
            // data.rustbox.0.print(2, 5, RB_NORMAL, White, Black, format!("Angle in deg:{}", pos.angle / 0.01745329252).as_str());
            //
            // let trace = data.field.trace(pos, None);
            // data.rustbox.0.print(2, 6, RB_NORMAL, White, Black, format!("Trace: X:{} Y:{}", (trace.0).0, (trace.0).1).as_str());
            // data.rustbox.0.print(2, 7, RB_NORMAL, White, Black, format!("Traced length:{}", trace.1).as_str());
            // data.rustbox.0.print(2, 8, RB_NORMAL, White, Black, format!("Traced tile:{}", trace.2).as_str());
        }
    }
}