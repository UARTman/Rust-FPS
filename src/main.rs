use std::default::Default;
use std::error::Error;

use rustbox::Color::{Black, White};
use rustbox::Key;
use rustbox::{Color, RustBox, RB_NORMAL};
use specs::{DispatcherBuilder, World, WorldExt};

use crate::ecs::resources::{GameStatus, KeyPresses, RustBoxWrapper};
use crate::ecs::systems::{KeyPressHandler, MessageRenderer, RBF};
use crate::util::init_rustbox;

mod ecs;
mod util;

fn main() {
    let mut world = World::new();
    world.insert(RustBoxWrapper(init_rustbox()));
    world.insert(GameStatus(true));
    world.insert(KeyPresses(Vec::new()));

    let mut dispatcher = DispatcherBuilder::new()
        .with(KeyPressHandler, "kph", &[])
        .with_thread_local(MessageRenderer)
        .with_thread_local(RBF)
        .build();

    dispatcher.setup(&mut world);
    dispatcher.dispatch(&mut world);

    loop {
        if world.read_resource::<GameStatus>().0 == false {
            break;
        };

        match world.read_resource::<RustBoxWrapper>().0.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                world.write_resource::<KeyPresses>().0.push(key.clone())
            }
            Err(e) => panic!("{}", e.to_string()),
            _ => {}
        }

        dispatcher.dispatch(&mut world);
    }
}
