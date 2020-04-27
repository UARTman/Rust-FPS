use std::default::Default;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};


use specs::{Builder, DispatcherBuilder, World, WorldExt};

use crate::ecs::components::Player;
use crate::ecs::components::position::Position;
use crate::ecs::resources::{Clock, Counter, GameField, GameStatus};
use crate::ecs::resources::event::{Event, Events};
use crate::ecs::resources::event::Event::{MoveDown, MoveLeft, MoveRight, MoveUp, Quit};
use crate::ecs::systems::{HudRenderer, MessageRenderer, PMoveHandler, QuitHandler, RBF, RaycastRenderer};
use crossterm::event::{poll, read, KeyEvent, KeyCode};
use std::io::{stdout, Write};
use crossterm::ExecutableCommand;
use crossterm::terminal::{Clear, SetSize, enable_raw_mode};
use crossterm::terminal::ClearType::All;

mod ecs;
mod util;

fn main() {
    let mut world = World::new();
    enable_raw_mode();

    stdout().execute(SetSize(180, 60));
    stdout().execute(Clear(All));
    stdout().execute(crossterm::cursor::Hide);
    stdout().flush();

    // let rust_box = Arc::new(init_rustbox());

    world.register::<Player>();
    world.register::<Position>();
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
        // stdout().execute(Clear(All));

        let new_now = Instant::now();
        let delta_time = new_now.duration_since(now).as_secs_f32();
        if delta_time < (1.0 / 60.0) { continue; }
        now = new_now;
        world.write_resource::<Clock>().delta_time = delta_time;

        if poll(Duration::from_millis(10)).unwrap() {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read().unwrap() {
                crossterm::event::Event::Key(event) => {
                    match event.code {
                        KeyCode::Char('w') => {
                            world.write_resource::<Events>().last_event = Some(MoveUp)
                        },
                        KeyCode::Char('s') => {
                            world.write_resource::<Events>().last_event = Some(MoveDown);
                        }
                        KeyCode::Char('a') => {
                            world.write_resource::<Events>().last_event = Some(MoveLeft);
                        }
                        KeyCode::Char('d') => {
                            world.write_resource::<Events>().last_event = Some(MoveRight);
                        }
                        KeyCode::Char('q') => {
                            world.write_resource::<Events>().last_event = Some(Quit);
                        }
                        _ => {}
                    }

                },
                crossterm::event::Event::Mouse(event) => {},
                crossterm::event::Event::Resize(width, height) => {},
            }
        }


        dispatcher.dispatch(&world);
        world.maintain();

        if !world.read_resource::<GameStatus>().0 { break; }
    }
}
