use crate::controller::state::State;

pub const BTN_SELECT_ALL: isize = 0;
pub const BTN_SECTION: isize = -1;

pub fn get_index_range(state: &State) -> (isize, isize) {
    (0, (state.m_files.len()).try_into().expect("Cannot convert length to isize"))
}