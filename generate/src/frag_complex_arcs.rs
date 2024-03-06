use crate::frag_155::{a144, a155};
use crate::frag_arc_halfs::{a45_24_half, a45_35_half};
use crate::frag_arc_long::{a45_24_long, a45_35_long};

pub fn n45_14_35(invert: bool) -> String {
    return format!("{}{}", a144(invert), a45_35_long(invert));
}

// TODO: Could be a large arc instead
pub fn n45_24_35(invert: bool) -> String {
    return format!("{}{}", a45_35_long(invert), a45_24_long(invert));
}

pub fn n45_24_15(invert: bool) -> String {
    return format!("{}{}", a155(invert), a45_24_long(invert));
}

pub fn n45_24(invert: bool) -> String {
    return format!("{}{}", a45_24_long(invert), a45_24_half(invert));
}

pub fn n45_35(invert: bool) -> String {
    return format!("{}{}", a45_35_long(invert), a45_35_half(invert));
}
