use specs::prelude::*;
use crate::ecs::resources::{GameField, RustBoxWrapper};
use rustbox::RB_NORMAL;
use rustbox::Color::{White, Black};

pub struct HudRenderer;

#[derive(SystemData)]
pub struct HudRendererData<'a>{
    field: Read<'a, GameField>,
    rustbox: Read<'a, RustBoxWrapper>
}

impl<'a> System<'a> for HudRenderer{
    type SystemData = HudRendererData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        for (i, row) in data.field.field.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                data.rustbox.0.print_char(
                    i,
                    j,
                    RB_NORMAL,
                    White,
                    Black,
                    *col
                )
            }
        }
    }
}