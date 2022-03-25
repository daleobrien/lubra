use crate::renders::constants::{H, M, Q1, Q3, U, W, Z};
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

pub const T1_V_ARC: Segment = Segment {
    centre: Point { x: M, y: Z },
    radius: W,
    segment: 0,
};
pub const T1_A_ARC: Segment = Segment {
    centre: Point { x: M, y: H },
    radius: W,
    segment: 0,
};

pub const T1_V: Segment = Segment {
    centre: Point { x: M, y: Z },
    radius: W,
    segment: 1,
};
pub const T1_A: Segment = Segment {
    centre: Point { x: M, y: H },
    radius: W,
    segment: 4,
};

pub const T2_ARC: Segment = Segment {
    centre: Point { x: Q3, y: H / 2.0 },
    radius: W,
    segment: 0,
};

pub const T2_V: Segment = Segment {
    centre: Point { x: Q3, y: H / 2.0 },
    radius: W,
    segment: 3,
};
pub const T2_A: Segment = Segment {
    centre: Point { x: Q3, y: H / 2.0 },
    radius: W,
    segment: 2,
};

pub const T3_ARC: Segment = Segment {
    centre: Point { x: Q1, y: H / 2.0 },
    radius: W,
    segment: 0,
};

pub const T3_V: Segment = Segment {
    centre: Point { x: Q1, y: H / 2.0 },
    radius: W,
    segment: 5,
};
pub const T3_A: Segment = Segment {
    centre: Point { x: Q1, y: H / 2.0 },
    radius: W,
    segment: 6,
};

// pub const M1_V: Segment = Segment {
//     centre: Point { x: M, y: D },
//     radius: W,
//     segment: 1,
// };
// pub const M1_A: Segment = Segment {
//     centre: Point { x: M, y: H - D },
//     radius: W,
//     segment: 4,
// };
// pub const M2_V: Segment = Segment {
//     centre: Point { x: M, y: D },
//     radius: W,
//     segment: 3,
// };
// pub const M2_A: Segment = Segment {
//     centre: Point { x: M, y: H - D },
//     radius: W,
//     segment: 2,
// };
// pub const M3_V: Segment = Segment {
//     centre: Point { x: M, y: D },
//     radius: W,
//     segment: 5,
// };
// pub const M3_A: Segment = Segment {
//     centre: Point { x: M, y: H - D },
//     radius: W,
//     segment: 6,
// };

pub const N1_A: Segment = Segment {
    centre: Point { x: M, y: Z },
    radius: W,
    segment: 1,
};
pub const N1_V: Segment = Segment {
    centre: Point { x: M, y: H },
    radius: W,
    segment: 4,
};
pub const N2_V: Segment = Segment {
    centre: Point { x: Z, y: Z },
    radius: W,
    segment: 6,
};
pub const N2_A: Segment = Segment {
    centre: Point { x: Z, y: H },
    radius: W,
    segment: 5,
};

pub const N3_V: Segment = Segment {
    centre: Point { x: U, y: Z },
    radius: W,
    segment: 2,
};
pub const N3_A: Segment = Segment {
    centre: Point { x: U, y: H },
    radius: W,
    segment: 3,
};
