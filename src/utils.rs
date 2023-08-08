use crate::controller::state::State;

pub const BTN_SELECT_ALL: isize = 0;
pub const BTN_SECTION: isize = -1;

pub fn get_index_range(state: &State) -> (isize, isize) {
    (BTN_SECTION, (state.m_files.len() - 1).try_into().expect("Cannot convert length to isize"))
}