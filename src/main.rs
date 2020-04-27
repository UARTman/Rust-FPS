use std::default::Default;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use rustbox::{Color, RB_NORMAL, RustBox};
use rustbox::Color::{Black, White};
use rustbox::Key;
use specs::{Builder, DispatcherBuilder, World, WorldExt};

use crate::ecs::components::Player;
use crate::ecs::components::position::Position;
use crate::ecs::resources::{Clock, Counter, GameField, GameStatus, KeyPresses, RustBoxWrapper, RendererRes};
use crate::ecs::resources::event::{Event, Events};
use crate::ecs::resources::event::Event::{MoveDown, MoveLeft, MoveRight, MoveUp, Quit};
use crate::ecs::systems::{HudRenderer, MessageRenderer, PMoveHandler, QuitHandler, RBF, RaycastRenderer};
use crate::util::init_rustbox;
use crate::renderer::termbox::TermboxRenderer;

mod ecs;
mod util;

fn main() {
    let mut world = World::new();

    let rust_box = Arc::new(init_rustbox());

    world.register::<Player>();
    world.register::<Position>();
    world.insert(RustBoxWrapper(Arc::clone(&rust_box)));
    world.insert(GameStatus(true));
    world.insert(GameField::new());
    world.insert(Clock::default());
    world.insert(Events::default());

    world.create_entity().with(Player).with(Position { x: 5.0, y: 5.0, angle: 0.0 }).build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(QuitHandler, "qh", &[])
        .with(PMoveHandler, "pmh", &[])
        .with_thread_local(RaycastRenderer)
        .with_thread_local(HudRenderer)
        .with_thread_local(MessageRenderer)
        .with_thread_local(RBF)
        .build();

    dispatcher.setup(&mut world);

    let mut now = Instant::now();


    loop {
        rust_box.clear();
        let new_now = Instant::now();
        let delta_time = new_now.duration_since(now).as_secs_f32();
        if delta_time < (1.0 / 60.0) { continue; }
        now = new_now;
        world.write_resource::<Clock>().delta_time = delta_time;

        match rust_box.peek_event(Duration::new(0, 10), false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('w') => {
                        world.write_resource::<Events>().last_event = Some(MoveUp);
                    }
                    Key::Char('s') => {
                        world.write_resource::<Events>().last_event = Some(MoveDown);
                    }
                    Key::Char('a') => {
                        world.write_resource::<Events>().last_event = Some(MoveLeft);
                    }
                    Key::Char('d') => {
                        world.write_resource::<Events>().last_event = Some(MoveRight);
                    }
                    Key::Char('q') => {
                        world.write_resource::<Events>().last_event = Some(Quit);
                    }
                    _ => {}
                }
            }
            Ok(rustbox::Event::ResizeEvent(_, _)) => {}
            Err(e) => panic!("{}", e.to_string()),
            _ => {}
        }

        dispatcher.dispatch(&world);
        world.maintain();

        if !world.read_resource::<GameStatus>().0 { break; }
    }
}
