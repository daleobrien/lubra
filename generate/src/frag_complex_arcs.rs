use crate::frag_155::{a144, a155};
use crate::frag_arc_long::a45_35_long;

pub fn a_14_45_35(invert: bool) -> String {
    return format!("{}{}", a144(invert), a45_35_long(invert));
}

pub fn a_14_45_15(invert: bool) -> String {
    return format!("{}{}", a144(invert), a155(invert));
}
