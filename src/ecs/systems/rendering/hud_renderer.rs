use rustbox::Color::{Black, White, Red};
use rustbox::RB_NORMAL;
use specs::prelude::*;

use crate::ecs::components::Player;
use crate::ecs::components::position::Position;
use crate::ecs::resources::{GameField, RustBoxWrapper};

pub struct HudRenderer;

#[derive(SystemData)]
pub struct HudRendererData<'a> {
    field: Read<'a, GameField>,
    rustbox: Read<'a, RustBoxWrapper>,
    player: ReadStorage<'a, Player>,
    position: ReadStorage<'a, Position>,
}

impl<'a> System<'a> for HudRenderer {
    type SystemData = HudRendererData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let (map_start_x, map_start_y) = if data.rustbox.0.width() < 12 || data.rustbox.0.height() < 12 {
            (0, 0)
        } else {
            (data.rustbox.0.width() - 12, data.rustbox.0.height() - 12)
        };

        for (i, row) in data.field.field.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                data.rustbox.0.print_char(
                    map_start_x + i,
                    map_start_y + j,
                    RB_NORMAL,
                    White,
                    Black,
                    *col,
                )
            }
        }

        for (player, position) in (&data.player, &data.position).join() {
            data.rustbox.0.print_char(
                map_start_x + position.x as usize,
                map_start_y + position.y as usize,
                RB_NORMAL,
                White,
                Black,
                '@',
            );
            let ((px, py), _, _) = data.field.trace(position, None);
            data.rustbox.0.print_char(
                map_start_x + px as usize,
                map_start_y + py as usize,
                RB_NORMAL,
                Red,
                Black,
                '*'
            )
        }


    }
}