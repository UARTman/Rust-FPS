use specs::{Component, VecStorage};

pub mod position;

pub struct Player;

impl Component for Player {
    type Storage = VecStorage<Self>;
}
