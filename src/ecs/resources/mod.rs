use std::rc::Rc;
use std::sync::{Arc, Mutex};

use rustbox::{Key, RustBox};

pub use input_handling::*;

use crate::util::init_rustbox;
use std::ops::{Div, Mul};
use crate::ecs::components::position::Position;

pub mod input_handling;
pub mod event;

pub struct RustBoxWrapper(pub Arc<RustBox>);

impl Default for RustBoxWrapper {
    fn default() -> Self {
        RustBoxWrapper(Arc::new(init_rustbox()))
    }
}

pub struct GameStatus(pub bool);

impl Default for GameStatus {
    fn default() -> Self {
        GameStatus(true)
    }
}


#[derive(Default)]
pub struct GameField {
    pub field: Vec<Vec<char>>
}

impl GameField {

    pub fn new() -> Self {
        let field = vec!(
            Vec::from("############"),
            Vec::from("##         #"),
            Vec::from("#          #"),
            Vec::from("#          #"),
            Vec::from("#          #"),
            Vec::from("#          #"),
            Vec::from("#          #"),
            Vec::from("#          #"),
            Vec::from("#          #"),
            Vec::from("#          #"),
            Vec::from("#          #"),
            Vec::from("############"),
        );
        let field_char = field.iter()
            .map(|x| x.iter()
                .map(|y| *y as char).collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        GameField { field: field_char }
    }

    pub fn trace(&self, pos: &Position, angle: Option<f32>) -> ((f32, f32), f32, char){
        let actual_angle = angle.unwrap_or(pos.angle);
        let step = 0.01f32;
        let mut pos_x = pos.x;
        let mut pos_y = pos.y;
        let mut moved = step;
        loop {
            pos_x = pos.x + moved * actual_angle.cos();
            pos_y = pos.y + moved * actual_angle.sin();

            let current_cell = self.field[pos_x as usize][pos_y as usize];
            match current_cell {
                '#' => {
                    return ((pos_x, pos_y), moved, '#')
                }
                _ => {}
            }

            moved += step;
        }
    }
}

#[derive(Default)]
pub struct Clock {
    pub delta_time: f32
}

impl Clock {
    pub fn fps(&self) -> f32 {
        1.0 / self.delta_time
    }
    pub fn unbind(&self, x: f32) -> f32 {
        return x * self.delta_time * 3.0
    }
}

#[derive(Default)]
pub struct Counter {
    pub cnt: f32
}