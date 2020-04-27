use specs::prelude::*;

use crate::ecs::resources::RustBoxWrapper;

pub struct RBF;

#[derive(SystemData)]
pub struct RBFData<'a> {
    pub rustbox: Read<'a, RustBoxWrapper>,
}

impl<'a> System<'a> for RBF {
    type SystemData = RBFData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        data.rustbox.0.present();
    }
}
