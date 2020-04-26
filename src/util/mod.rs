use rustbox::RustBox;
// use rustbox::OutputMode::Normal;

pub fn init_rustbox() -> RustBox {
    match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    }
}
