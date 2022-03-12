use crate::renders::constants::{C, D, H, M, Q1, Q3, U, W, Z};
use crate::renders::primatives::{Line, Point, Segment};

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

pub const S1_A: Line = Line {
    start: Point { x: M, y: H },
    delta: Point { x: Z, y: -D },
};

pub const S1_V: Line = Line {
    start: Point { x: M, y: Z },
    delta: Point { x: Z, y: D },
};

pub const S2_A: Line = Line {
    start: Point { x: Q3, y: C },
    delta: Point { x: -Q1, y: C - D },
};

pub const S2_V: Line = Line {
    start: Point { x: Q1, y: H - C },
    delta: Point { x: Q1, y: D - C },
};

pub const S3_A: Line = Line {
    start: Point { x: Q1, y: C },
    delta: Point { x: Q1, y: C - D },
};

pub const S3_V: Line = Line {
    start: Point { x: Q3, y: H - C },
    delta: Point { x: -Q1, y: D - C },
};

pub const L1_A: Line = Line {
    start: Point { x: M, y: Z },
    delta: Point { x: Z, y: H },
};

pub const L1_V: Line = Line {
    start: Point { x: M, y: Z },
    delta: Point { x: Z, y: H },
};

pub const L2_A: Line = Line {
    start: Point { x: Q3, y: C },
    delta: Point { x: -Q3, y: H - C },
};

pub const L3_V: Line = Line {
    start: Point { x: Q1, y: H - C },
    delta: Point { x: Q3, y: C - H },
};

pub const L3_A: Line = Line {
    start: Point { x: Q1, y: C },
    delta: Point { x: Q3, y: H - C },
};

pub const L2_V: Line = Line {
    start: Point { x: Q3, y: H - C },
    delta: Point { x: -Q3, y: C - H },
};

pub const M1_V: Segment = Segment {
    centre: Point { x: M, y: D },
    radius: W,
    segment: 1,
};
pub const M1_A: Segment = Segment {
    centre: Point { x: M, y: H - D },
    radius: W,
    segment: 4,
};
pub const M2_V: Segment = Segment {
    centre: Point { x: M, y: D },
    radius: W,
    segment: 5,
};
pub const M2_A: Segment = Segment {
    centre: Point { x: M, y: H - D },
    radius: W,
    segment: 2,
};
pub const M3_V: Segment = Segment {
    centre: Point { x: M, y: D },
    radius: W,
    segment: 3,
};
pub const M3_A: Segment = Segment {
    centre: Point { x: M, y: H - D },
    radius: W,
    segment: 6,
};

pub const A1_V: Segment = Segment {
    centre: Point { x: M, y: H },
    radius: M,
    segment: 5,
};
pub const A1_A: Segment = Segment {
    centre: Point { x: M, y: Z },
    radius: M,
    segment: 2,
};

pub const A2_V: Segment = Segment {
    centre: Point { x: Z, y: Z },
    radius: M,
    segment: 1,
};
pub const A2_A: Segment = Segment {
    centre: Point { x: Z, y: H },
    radius: M,
    segment: 6,
};

pub const A3_V: Segment = Segment {
    centre: Point { x: U, y: Z },
    radius: M,
    segment: 3,
};
pub const A3_A: Segment = Segment {
    centre: Point { x: U, y: H },
    radius: M,
    segment: 4,
};
