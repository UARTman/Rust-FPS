use std::f32::consts::PI;

use specs::prelude::*;

use crate::ecs::components::Player;
use crate::ecs::components::position::Position;
use crate::ecs::resources::{GameField};
use crate::util::moveto;
use std::io::{stdout, Write};

pub struct RaycastRenderer;

#[derive(SystemData)]
pub struct RaycastRenderData<'a> {
    pub field: Read<'a, GameField>,
    pub player: ReadStorage<'a, Player>,
    pub position: ReadStorage<'a, Position>,
}

impl<'a> System<'a> for RaycastRenderer {
    type SystemData = RaycastRenderData<'a>;


    fn run(&mut self, data: Self::SystemData) {
        let width = 120;
        let height = 40;

        for (_, player_pos) in (&data.player, &data.position).join() {
            let mut render_buffer: Vec<Vec<char>> = Vec::new();

            for i in 0..width {
                let mut col: Vec<char> = Vec::new();
                let desired_angle = player_pos.angle - (PI / 3.0) + i as f32 * ((PI / 1.5)  / width as f32);
                let ((tx, ty), distance, _) = data.field.trace(player_pos, Some(desired_angle));

                let mut fill = { if distance < 2.0 { '█' } else if distance < 6.0 { '▓' } else if distance < 8.0 { '▒' } else if distance < 12.0 { '░' } else { ' ' } };
                let margin = (distance * 1.2) as usize;


                for _ in 0..margin {
                    col.push('^');
                }

                for _ in 0..height - 2 * margin {
                    col.push(fill);
                }
                for _ in 0..margin {
                    col.push('_')
                }

                render_buffer.push(col);
            }

            for i in 0..width {
                for j in 0..height {
                    moveto(i,j);
                    print!("{}", render_buffer[i][j]);
                    // stdout().flush();
                }
            }
        }
    }
}

