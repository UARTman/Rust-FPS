extern crate specs;

use std::io::{stdout, Write};

use specs::prelude::*;

use crate::ecs::resources::{Clock, Counter};
use crate::util::moveto;

use self::specs::Read;

pub struct MessageRenderer;

#[derive(SystemData)]
pub struct MessageRendererData<'a> {
    pub clock: Read<'a, Clock>,
}

impl<'a> System<'a> for MessageRenderer {
    type SystemData = MessageRendererData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        moveto(150, 14);
        print!("FPS:{}", data.clock.fps());
        // stdout().flush();
    }
}
