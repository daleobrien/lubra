use crate::fragments::frag_arc_halfs::{
    a45_24_half, a45_35_half, a46_14_half, a46_36_half, a56_15_half, a56_26_half,
};
use crate::fragments::frag_arc_long::{
    a45_24_long, a45_35_long, a46_14_long, a46_36_long, a56_15_long, a56_26_long,
};

// n45_24_35, n46_14_36 and n46_15_26 are in frag_crescent

// 4-5 (n45_14_35 and n45_24_15 are in frag_s_curve)
pub fn n45_24(invert: bool) -> String {
    format!("{}{}", a45_24_long(invert), a45_24_half(invert))
}
pub fn n45_35(invert: bool) -> String {
    format!("{}{}", a45_35_long(invert), a45_35_half(invert))
}

// 4-6 (n46_14_26 and n46_24_36 are in frag_s_curve)
pub fn n46_14(invert: bool) -> String {
    format!("{}{}", a46_14_long(invert), a46_14_half(invert))
}
pub fn n46_35(invert: bool) -> String {
    format!("{}{}", a46_36_half(invert), a46_36_long(invert))
}

// 5-6 (n56_35_26 and n56_15_36 are in frag_s_curve)
pub fn n56_15(invert: bool) -> String {
    format!("{}{}", a56_15_long(invert), a56_15_half(invert))
}
pub fn n56_26(invert: bool) -> String {
    format!("{}{}", a56_26_long(invert), a56_26_half(invert))
}
