use specs::prelude::*;

use std::io::{stdout, Write};
use crossterm::ExecutableCommand;
use crossterm::terminal::ScrollUp;

pub struct RBF;



impl<'a> System<'a> for RBF {
    type SystemData = ();

    fn run(&mut self, data: Self::SystemData) {
        // stdout().execute(ScrollUp(40));
        stdout().flush();
    }
}
