use crate::renders::constants::{D, H, M, U};
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
