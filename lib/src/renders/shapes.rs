use crate::renders::constants::{C, D, H, M, Q1, Q3, U};
use crate::renders::primatives::{Line, Point};

pub const TRIANGLE_A: [Line; 3] = [
    Line {
        start: Point { x: 0.0, y: H },
        delta: Point { x: U, y: 0.0 },
    },
    Line {
        start: Point { x: U, y: H },
        delta: Point { x: -M, y: -H },
    },
    Line {
        start: Point { x: M, y: 0.0 },
        delta: Point { x: -M, y: H },
    },
];

pub const TRIANGLE_V: [Line; 3] = [
    Line {
        start: Point { x: 0.0, y: 0.0 },
        delta: Point { x: U, y: 0.0 },
    },
    Line {
        start: Point { x: U, y: 0.0 },
        delta: Point { x: -M, y: H },
    },
    Line {
        start: Point { x: M, y: H },
        delta: Point { x: -M, y: -H },
    },
];

pub const SHORT_S1_A: Line = Line {
    start: Point { x: M, y: H },
    delta: Point { x: 0.0, y: -D },
};

pub const SHORT_S1_V: Line = Line {
    start: Point { x: M, y: 0.0 },
    delta: Point { x: 0.0, y: D },
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
