use crate::renders::primatives::{Line, Point};

pub const U: f64 = 120.0;
pub const M: f64 = 60.0;
pub const Q1: f64 = 30.0;
pub const Q3: f64 = 90.0;
pub const Z: f64 = 0.0;

pub const W: f64 = 20.0;

// U * f64::sqrt(3.0) / 2.0;
pub const H: f64 = 103.92304845413263761;

// U * f64::sqrt(3.0) / 6.0;
pub const D: f64 = 34.641016151377545871;

pub const P1: Point = Point { x: 0., y: 0. };
pub const P2: Point = Point { x: 40., y: 0. };
pub const P3: Point = Point { x: 80., y: 0. };
pub const P4: Point = Point { x: 120., y: 0. };
pub const P5: Point = Point { x: 60., y: 103.923 };
pub const P6: Point = Point { x: 60., y: 34.641 };
pub const P7: Point = Point { x: 20., y: 34.641 };
pub const P8: Point = Point { x: 40., y: 69.282 };
pub const P9: Point = Point { x: 100., y: 34.641 };
pub const P10: Point = Point { x: 80., y: 69.282 };
pub const P11: Point = Point { x: 40., y: 34.641 };
pub const P12: Point = Point { x: 80., y: 34.641 };
pub const P13: Point = Point { x: 40., y: 26.4634 };
pub const P14: Point = Point { x: 80., y: 26.4634 };
pub const P15: Point = Point { x: 50., y: 51.9615 };
pub const P16: Point = Point { x: 70., y: 17.3205 };
pub const P17: Point = Point { x: 70., y: 51.9615 };
pub const P18: Point = Point { x: 50., y: 17.3205 };
pub const P19: Point = Point {
    x: 62.918,
    y: 56.0503,
};
pub const P20: Point = Point {
    x: 57.082,
    y: 56.0503,
};
pub const P21: Point = Point {
    x: 42.918,
    y: 21.4093,
};
pub const P22: Point = Point {
    x: 77.082,
    y: 21.4093,
};
pub const P23: Point = Point {
    x: 130.,
    y: 17.3205,
};
pub const P24: Point = Point {
    x: 110.,
    y: -17.3205,
};
pub const P25: Point = Point {
    x: -10.,
    y: 17.3205,
};
pub const P26: Point = Point {
    x: 10.,
    y: -17.3205,
};
pub const P27: Point = Point { x: 40., y: 103.923 };
pub const P28: Point = Point { x: 80., y: 103.923 };
pub const P29: Point = Point { x: 0., y: 103.923 };
pub const P30: Point = Point {
    x: 120.,
    y: 103.923,
};
pub const P31: Point = Point { x: 60., y: 0. };
pub const P32: Point = Point { x: 60., y: 69.282 };
pub const P33: Point = Point { x: 20., y: 69.282 };
pub const P34: Point = Point { x: 40., y: 34.641 };
pub const P35: Point = Point { x: 100., y: 69.282 };
pub const P36: Point = Point { x: 80., y: 34.641 };
pub const P37: Point = Point { x: 40., y: 69.282 };
pub const P38: Point = Point { x: 80., y: 69.282 };
pub const P39: Point = Point { x: 40., y: 77.4597 };
pub const P40: Point = Point { x: 80., y: 77.4597 };
pub const P41: Point = Point { x: 70., y: 86.6025 };
pub const P42: Point = Point { x: 50., y: 86.6025 };
pub const P43: Point = Point {
    x: 62.918,
    y: 47.8727,
};
pub const P44: Point = Point {
    x: 57.082,
    y: 47.8727,
};
pub const P45: Point = Point {
    x: 42.918,
    y: 82.5137,
};
pub const P46: Point = Point {
    x: 77.082,
    y: 82.5137,
};
pub const P47: Point = Point {
    x: 130.,
    y: 86.6025,
};
pub const P48: Point = Point {
    x: 110.,
    y: 121.244,
};
pub const P49: Point = Point {
    x: -10.,
    y: 86.6025,
};
pub const P50: Point = Point { x: 10., y: 121.244 };

pub const L01: Line = Line {
    start: Point { x: 40., y: 0. },
    delta: Point {
        x: -30.,
        y: -17.3205,
    },
};
pub const L02: Line = Line {
    start: Point {
        x: -10.,
        y: 17.3205,
    },
    delta: Point { x: 30., y: 17.3205 },
};
pub const L03: Line = Line {
    start: Point { x: 80., y: 69.282 },
    delta: Point {
        x: -30.,
        y: -17.3205,
    },
};
pub const L04: Line = Line {
    start: Point { x: 100., y: 34.641 },
    delta: Point {
        x: -30.,
        y: -17.3205,
    },
};
pub const L05: Line = Line {
    start: Point { x: 40., y: 0. },
    delta: Point { x: 0., y: 34.641 },
};
pub const L06: Line = Line {
    start: Point { x: 80., y: 0. },
    delta: Point { x: 0., y: 34.641 },
};
pub const L07: Line = Line {
    start: Point { x: 40., y: 69.282 },
    delta: Point { x: 0., y: 34.641 },
};
pub const L08: Line = Line {
    start: Point { x: 80., y: 103.923 },
    delta: Point { x: 0., y: -34.641 },
};
pub const L09: Line = Line {
    start: Point { x: 40., y: 69.282 },
    delta: Point {
        x: 30.,
        y: -17.3205,
    },
};
pub const L10: Line = Line {
    start: Point { x: 20., y: 34.641 },
    delta: Point {
        x: 30.,
        y: -17.3205,
    },
};
pub const L11: Line = Line {
    start: Point { x: 100., y: 34.641 },
    delta: Point {
        x: 30.,
        y: -17.3205,
    },
};
pub const L12: Line = Line {
    start: Point {
        x: 110.,
        y: -17.3205,
    },
    delta: Point {
        x: -30.,
        y: 17.3205,
    },
};
pub const L13: Line = Line {
    start: Point { x: 20., y: 34.641 },
    delta: Point {
        x: 22.918,
        y: -13.2317,
    },
};
pub const L14: Line = Line {
    start: Point { x: 40., y: 69.282 },
    delta: Point {
        x: 22.918,
        y: -13.2317,
    },
};
pub const L15: Line = Line {
    start: Point { x: 80., y: 69.282 },
    delta: Point {
        x: -22.918,
        y: -13.2317,
    },
};
pub const L16: Line = Line {
    start: Point { x: 100., y: 34.641 },
    delta: Point {
        x: -22.918,
        y: -13.2317,
    },
};
pub const L17: Line = Line {
    start: Point { x: 80., y: 0. },
    delta: Point { x: 0., y: 26.4634 },
};
pub const L18: Line = Line {
    start: Point { x: 40., y: 0. },
    delta: Point { x: 0., y: 26.4634 },
};
pub const L19: Line = Line {
    start: Point { x: 20., y: 34.641 },
    delta: Point {
        x: 37.082,
        y: 21.4093,
    },
};
pub const L20: Line = Line {
    start: Point { x: 40., y: 0. },
    delta: Point {
        x: 37.082,
        y: 21.4093,
    },
};
pub const L21: Line = Line {
    start: Point { x: 40., y: 26.4634 },
    delta: Point { x: 0., y: 42.8187 },
};
pub const L22: Line = Line {
    start: Point { x: 80., y: 26.4634 },
    delta: Point { x: 0., y: 42.8187 },
};
pub const L23: Line = Line {
    start: Point { x: 80., y: 0. },
    delta: Point {
        x: -37.082,
        y: 21.4093,
    },
};
pub const L24: Line = Line {
    start: Point { x: 100., y: 34.641 },
    delta: Point {
        x: -37.082,
        y: 21.4093,
    },
};
pub const L25: Line = Line {
    start: Point { x: 40., y: 103.923 },
    delta: Point {
        x: -30.,
        y: 17.3205,
    },
};
pub const L26: Line = Line {
    start: Point {
        x: -10.,
        y: 86.6025,
    },
    delta: Point {
        x: 30.,
        y: -17.3205,
    },
};
pub const L27: Line = Line {
    start: Point { x: 80., y: 34.641 },
    delta: Point {
        x: -30.,
        y: 17.3205,
    },
};
pub const L28: Line = Line {
    start: Point { x: 100., y: 69.282 },
    delta: Point {
        x: -30.,
        y: 17.3205,
    },
};
pub const L29: Line = Line {
    start: Point { x: 40., y: 103.923 },
    delta: Point { x: 0., y: -34.641 },
};
pub const L30: Line = Line {
    start: Point { x: 80., y: 103.923 },
    delta: Point { x: 0., y: -34.641 },
};
pub const L31: Line = Line {
    start: Point { x: 40., y: 34.641 },
    delta: Point { x: 0., y: -34.641 },
};
pub const L32: Line = Line {
    start: Point { x: 80., y: 0. },
    delta: Point { x: 0., y: 34.641 },
};
pub const L33: Line = Line {
    start: Point { x: 40., y: 34.641 },
    delta: Point { x: 30., y: 17.3205 },
};
pub const L34: Line = Line {
    start: Point { x: 20., y: 69.282 },
    delta: Point { x: 30., y: 17.3205 },
};
pub const L35: Line = Line {
    start: Point { x: 100., y: 69.282 },
    delta: Point { x: 30., y: 17.3205 },
};
pub const L36: Line = Line {
    start: Point {
        x: 110.,
        y: 121.244,
    },
    delta: Point {
        x: -30.,
        y: -17.3205,
    },
};
pub const L37: Line = Line {
    start: Point { x: 20., y: 69.282 },
    delta: Point {
        x: 22.918,
        y: 13.2317,
    },
};
pub const L38: Line = Line {
    start: Point { x: 40., y: 34.641 },
    delta: Point {
        x: 22.918,
        y: 13.2317,
    },
};
pub const L39: Line = Line {
    start: Point { x: 80., y: 34.641 },
    delta: Point {
        x: -22.918,
        y: 13.2317,
    },
};
pub const L40: Line = Line {
    start: Point { x: 100., y: 69.282 },
    delta: Point {
        x: -22.918,
        y: 13.2317,
    },
};
pub const L41: Line = Line {
    start: Point { x: 80., y: 103.923 },
    delta: Point { x: 0., y: -26.4634 },
};
pub const L42: Line = Line {
    start: Point { x: 40., y: 103.923 },
    delta: Point { x: 0., y: -26.4634 },
};
pub const L43: Line = Line {
    start: Point { x: 20., y: 69.282 },
    delta: Point {
        x: 37.082,
        y: -21.4093,
    },
};
pub const L44: Line = Line {
    start: Point { x: 40., y: 103.923 },
    delta: Point {
        x: 37.082,
        y: -21.4093,
    },
};
pub const L45: Line = Line {
    start: Point { x: 40., y: 77.4597 },
    delta: Point { x: 0., y: -42.8187 },
};
pub const L46: Line = Line {
    start: Point { x: 80., y: 77.4597 },
    delta: Point { x: 0., y: -42.8187 },
};
pub const L47: Line = Line {
    start: Point { x: 80., y: 103.923 },
    delta: Point {
        x: -37.082,
        y: -21.4093,
    },
};
pub const L48: Line = Line {
    start: Point { x: 100., y: 69.282 },
    delta: Point {
        x: -37.082,
        y: -21.4093,
    },
};
