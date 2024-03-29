use super::*;

#[derive(Debug)]
pub struct Controller {
    pub Buttons: u8,
    pub shiftreg: u8,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            shiftreg: 0x0,
            Buttons: 0x0,
        }
    }
}