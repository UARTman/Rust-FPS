use std::io::{stdout, Write};

use crossterm::cursor::MoveTo;
use crossterm::ExecutableCommand;
use specs::prelude::*;

use crate::ecs::components::Player;
use crate::ecs::components::position::Position;
use crate::ecs::resources::{GameField};
use crate::util::moveto;
use crossterm::style::{SetForegroundColor, Color, ResetColor};

pub struct HudRenderer;

#[derive(SystemData)]
pub struct HudRendererData<'a> {
    field: Read<'a, GameField>,
    player: ReadStorage<'a, Player>,
    position: ReadStorage<'a, Position>,
}

impl<'a> System<'a> for HudRenderer {
    type SystemData = HudRendererData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let (map_start_x, map_start_y) = (150, 0);

        for (i, row) in data.field.field.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                moveto(map_start_x + i, map_start_y + j);
                stdout().write(&[*col as u8]);
                // stdout().flush();
            }
        }

        for (player, position) in (&data.player, &data.position).join() {
            moveto(map_start_x + position.x as usize, map_start_y + position.y as usize);
            stdout().write(&['@' as u8]);
            // stdout().flush();

            let ((px, py), _, _) = data.field.trace(position, None);
            moveto(map_start_x + px as usize, map_start_y + py as usize);
            stdout().execute(SetForegroundColor(Color::Red));
            stdout().write(&['*' as u8]);
            stdout().execute(ResetColor);
            // stdout().flush();
        }
    }
}