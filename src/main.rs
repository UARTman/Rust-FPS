use std::default::Default;
use std::error::Error;

use rustbox::{Color, RB_NORMAL, RustBox};
use rustbox::Color::{Black, White};
use rustbox::Key;
use specs::{DispatcherBuilder, World, WorldExt};

use crate::ecs::resources::{GameStatus, KeyPresses, RustBoxWrapper, GameField};
use crate::ecs::systems::{KeyPressHandler, MessageRenderer, RBF, HudRenderer};
use crate::util::init_rustbox;

mod ecs;
mod util;

fn main() {
    let mut world = World::new();
    world.insert(RustBoxWrapper(init_rustbox()));
    world.insert(GameStatus(true));
    world.insert(KeyPresses(Vec::new()));
    world.insert(GameField::new());

    let mut dispatcher = DispatcherBuilder::new()
        .with(KeyPressHandler, "kph", &[])
        .with_thread_local(HudRenderer)
        .with_thread_local(MessageRenderer)
        .with_thread_local(RBF)
        .build();

    dispatcher.setup(&mut world);
    dispatcher.dispatch(&mut world);

    while world.read_resource::<GameStatus>().0 {
        match world.read_resource::<RustBoxWrapper>().0.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                world.write_resource::<KeyPresses>().0.push(key)
            }
            Err(e) => panic!("{}", e.to_string()),
            _ => {}
        }

        dispatcher.dispatch(&mut world);
    }
}
