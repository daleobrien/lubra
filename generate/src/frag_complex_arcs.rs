use crate::frag_155::{a144, a155, a244, a266, a355, a366};
use crate::frag_arc_halfs::{a45_24_half, a45_35_half, a46_14_half, a46_36_half};
use crate::frag_arc_long::{
    a45_24_long, a45_35_long, a46_14_long, a46_36_long, a56_15_long, a56_26_long,
};

// TODO: These could be a large arc instead
pub fn n45_24_35(invert: bool) -> String {
    return format!("{}{}", a45_35_long(invert), a45_24_long(invert));
}
pub fn n46_14_36(invert: bool) -> String {
    return format!("{}{}", a46_14_long(invert), a46_36_long(invert));
}

// 4-5
pub fn n45_14_35(invert: bool) -> String {
    return format!("{}{}", a144(invert), a45_35_long(invert));
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

// 4-6
pub fn n46_14_26(invert: bool) -> String {
    return format!("{}{}", a266(invert), a46_14_long(invert));
}
pub fn n46_24_36(invert: bool) -> String {
    return format!("{}{}", a244(invert), a46_36_long(invert));
}
pub fn n46_14(invert: bool) -> String {
    return format!("{}{}", a46_14_long(invert), a46_14_half(invert));
}
pub fn n46_35(invert: bool) -> String {
    return format!("{}{}", a46_36_half(invert), a46_36_long(invert));
}

// 5-6
pub fn n56_35_26(invert: bool) -> String {
    return format!("{}{}", a355(invert), a56_26_long(invert));
}
pub fn n56_15_36(invert: bool) -> String {
    return format!("{}{}", a366(invert), a56_15_long(invert));
}
