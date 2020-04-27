extern crate specs;

use rustbox::Color::{Black, White};
use rustbox::RB_NORMAL;
use specs::prelude::*;

use crate::ecs::resources::{Clock, RustBoxWrapper, Counter};

use self::specs::Read;

pub struct MessageRenderer;

#[derive(SystemData)]
pub struct MessageRendererData<'a> {
    pub rustbox: Read<'a, RustBoxWrapper>,
    pub clock: Read<'a, Clock>,
}

impl<'a> System<'a> for MessageRenderer {
    type SystemData = MessageRendererData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let rb = data.rustbox;
        rb.0.print(
            0,
            1,
            RB_NORMAL,
            White,
            Black,
            format!("FPS:{}", data.clock.fps()).as_str()
        );
    }
}
