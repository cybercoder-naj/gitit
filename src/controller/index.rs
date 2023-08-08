pub const MASTER_FILES: u8 = 1;
pub const MASTER_BUTTONS: u8 = 2;
pub const BUTTON_SELECT_DESELECT_ALL: u8 = 0;

pub struct Index {
    master: u8,
    button: u8
}

impl Index {
    pub fn new() -> Self {
        Index {
            master: MASTER_FILES,
            button: BUTTON_SELECT_DESELECT_ALL
        }
    }
}