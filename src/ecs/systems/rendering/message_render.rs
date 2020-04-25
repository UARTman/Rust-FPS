extern crate specs;

use rustbox::Color::{Black, White};
use rustbox::RB_NORMAL;
use specs::prelude::*;

use crate::ecs::resources::RustBoxWrapper;

use self::specs::Read;

pub struct MessageRenderer;

#[derive(SystemData)]
pub struct MessageRendererData<'a> {
    pub rustbox: Read<'a, RustBoxWrapper>,
}

impl<'a> System<'a> for MessageRenderer {
    type SystemData = MessageRendererData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let rb = data.rustbox;
        rb.0.print(1, 1, RB_NORMAL, White, Black, "Displaying message...");
    }
}
