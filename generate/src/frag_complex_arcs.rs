use crate::frag_155::{a144, a155};
use crate::frag_arc_long::{a45_24_long, a45_35_long};

pub fn n45_14_35(invert: bool) -> String {
    return format!("{}{}", a144(invert), a45_35_long(invert));
}

// TODO: Delete this function:
pub fn a_14_45_15(invert: bool) -> String {
    return format!("{}{}", a144(invert), a155(invert));
}

pub fn n45_24_15(invert: bool) -> String {
    return format!("{}{}", a155(invert), a45_24_long(invert));
}
