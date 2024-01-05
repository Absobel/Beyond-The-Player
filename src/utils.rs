use crate::constants::*;

pub fn add_delta(pos: (u16, u16), delta: (i16, i16)) -> (u16, u16) {
    let mut to_check = (pos.0 as i16 + delta.0, pos.1 as i16 + delta.1);

    if to_check.0 < 0 {
        to_check.0 = LEVEL_X_MAX as i16 - 1;
    } else if to_check.0 >= LEVEL_X_MAX as i16 {
        to_check.0 = 0;
    }

    if to_check.1 < 0 {
        to_check.1 = LEVEL_Y_MAX as i16 - 1;
    } else if to_check.1 >= LEVEL_Y_MAX as i16 {
        to_check.1 = 0;
    }

    (to_check.0 as u16, to_check.1 as u16)
}
