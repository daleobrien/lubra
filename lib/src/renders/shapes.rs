use crate::renders::constants::{C, D, H, M, Q1, Q3, U, Z};
use crate::renders::primatives::{Line, Point};

pub const TRIANGLE_A: [Line; 3] = [
    Line {
        start: Point { x: Z, y: H },
        delta: Point { x: U, y: Z },
    },
    Line {
        start: Point { x: U, y: H },
        delta: Point { x: -M, y: -H },
    },
    Line {
        start: Point { x: M, y: Z },
        delta: Point { x: -M, y: H },
    },
];

pub const TRIANGLE_V: [Line; 3] = [
    Line {
        start: Point { x: Z, y: Z },
        delta: Point { x: U, y: Z },
    },
    Line {
        start: Point { x: U, y: Z },
        delta: Point { x: -M, y: H },
    },
    Line {
        start: Point { x: M, y: H },
        delta: Point { x: -M, y: -H },
    },
];

pub const SHORT_S1_A: Line = Line {
    start: Point { x: M, y: H },
    delta: Point { x: Z, y: -D },
};

pub const SHORT_S1_V: Line = Line {
    start: Point { x: M, y: Z },
    delta: Point { x: Z, y: D },
};

pub const SHORT_S2_A: Line = Line {
    start: Point { x: Q3, y: C },
    delta: Point { x: -Q1, y: C - D },
};

pub const SHORT_S2_V: Line = Line {
    start: Point { x: Q1, y: H - C },
    delta: Point { x: Q1, y: D - C },
};

pub const SHORT_S3_A: Line = Line {
    start: Point { x: Q1, y: C },
    delta: Point { x: Q1, y: C - D },
};

pub const SHORT_S3_V: Line = Line {
    start: Point { x: Q3, y: H - C },
    delta: Point { x: -Q1, y: D - C },
};

pub const LONG_L1_A: Line = Line {
    start: Point { x: M, y: Z },
    delta: Point { x: Z, y: H },
};

pub const LONG_L1_V: Line = Line {
    start: Point { x: M, y: Z },
    delta: Point { x: Z, y: H },
};

pub const LONG_L2_A: Line = Line {
    start: Point { x: Q3, y: C },
    delta: Point { x: -Q3, y: H - C },
};

pub const LONG_L3_V: Line = Line {
    start: Point { x: Q1, y: H - C },
    delta: Point { x: Q3, y: C - H },
};

pub const LONG_L3_A: Line = Line {
    start: Point { x: Q1, y: C },
    delta: Point { x: Q3, y: H - C },
};

pub const LONG_L2_V: Line = Line {
    start: Point { x: Q3, y: H - C },
    delta: Point { x: -Q3, y: C - H },
};
