use rustbox::Color::White;
use rustbox::OutputMode::Normal;
use rustbox::{Color, RustBox, Style, RB_NORMAL};

pub fn init_rustbox() -> RustBox {
    match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    }
}
