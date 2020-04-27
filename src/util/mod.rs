
use std::io::stdout;
use crossterm::ExecutableCommand;
use crossterm::cursor::MoveTo;

pub fn moveto(x: usize, y: usize) {
    stdout().execute(MoveTo(x as u16, y as u16));
}