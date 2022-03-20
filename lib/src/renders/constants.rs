use crate::renders::primatives::{Line, Point};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq)]
pub enum TileType {
    // S1,
    // S2,
    // S3,
    T1,
    T2,
    T3,
    A1,
    A2,
    A3,
    L1,
    L2,
    L3,
    C1,
    C2,
    C3,
    N1,
    N2,
    N3,
    Y1,
    Y2,
    Y3,
    X1,
    X2,
    X3,
    Z1,
}

pub const WALL: [&TileType; 6] = [
    &TileType::Z1,
    &TileType::X3,
    &TileType::X3,
    &TileType::X3,
    &TileType::X3,
    &TileType::X3,
];

pub const U: f64 = 120.0;
pub const Q1: f64 = 30.0;
pub const Q3: f64 = 90.0;
pub const M: f64 = 60.0;
pub const Z: f64 = 0.0;

pub const W: f64 = 20.0;

// U * f64::sqrt(3.0) / 2.0;
pub const H: f64 = 103.92304845413263761;

// U * f64::sqrt(3.0) / 6.0;
// pub const D: f64 = 34.641016151377545871;

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

pub const L07: Line = Line {
    start: Point { x: 40., y: 69.282 },
    delta: Point { x: 0., y: 34.641 },
};
pub const L08: Line = Line {
    start: Point { x: 80., y: 103.923 },
    delta: Point { x: 0., y: -34.641 },
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

pub const L31: Line = Line {
    start: Point { x: 40., y: 34.641 },
    delta: Point { x: 0., y: -34.641 },
};
pub const L32: Line = Line {
    start: Point { x: 80., y: 0. },
    delta: Point { x: 0., y: 34.641 },
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
pub const L49: Line = Line {
    start: Point { x: 40., y: 0. },
    delta: Point { x: 0., y: 103.923 },
};
pub const L50: Line = Line {
    start: Point { x: 40., y: 69.282 },
    delta: Point {
        x: 90.,
        y: -51.9615,
    },
};
pub const L51: Line = Line {
    start: Point { x: 80., y: 69.282 },
    delta: Point {
        x: -90.,
        y: -51.9615,
    },
};
pub const L52: Line = Line {
    start: Point { x: 80., y: 0. },
    delta: Point { x: 0., y: 103.923 },
};
pub const L53: Line = Line {
    start: Point { x: 20., y: 34.641 },
    delta: Point {
        x: 90.,
        y: -51.9615,
    },
};
pub const L54: Line = Line {
    start: Point { x: 100., y: 34.641 },
    delta: Point {
        x: -90.,
        y: -51.9615,
    },
};

pub const L56: Line = Line {
    start: Point { x: 40., y: 34.641 },
    delta: Point { x: 90., y: 51.9615 },
};
pub const L57: Line = Line {
    start: Point { x: 80., y: 34.641 },
    delta: Point {
        x: -90.,
        y: 51.9615,
    },
};

pub const L59: Line = Line {
    start: Point { x: 20., y: 69.282 },
    delta: Point { x: 90., y: 51.9615 },
};
pub const L60: Line = Line {
    start: Point { x: 100., y: 69.282 },
    delta: Point {
        x: -90.,
        y: 51.9615,
    },
};

pub const L62: Line = Line {
    start: Point { x: 40., y: 23.094 },
    delta: Point { x: -20., y: 11.547 },
};
pub const L63: Line = Line {
    start: Point { x: 80., y: 23.094 },
    delta: Point { x: 0., y: -23.094 },
};

pub const L66: Line = Line {
    start: Point { x: 60., y: 57.735 },
    delta: Point { x: 20., y: 11.547 },
};
pub const L67: Line = Line {
    start: Point { x: 40., y: 80.829 },
    delta: Point {
        x: -20.,
        y: -11.547,
    },
};

pub const L69: Line = Line {
    start: Point { x: 80., y: 80.829 },
    delta: Point { x: 0., y: 23.094 },
};

pub const L72: Line = Line {
    start: Point { x: 60., y: 46.188 },
    delta: Point { x: 20., y: -11.547 },
};

pub const L74: Line = Line {
    start: Point { x: 60., y: 11.547 },
    delta: Point {
        x: -50.,
        y: -28.8675,
    },
};

pub const L76: Line = Line {
    start: Point { x: 40., y: 46.188 },
    delta: Point { x: 0., y: 57.735 },
};
pub const L77: Line = Line {
    start: Point { x: 80., y: 46.188 },
    delta: Point {
        x: 50.,
        y: -28.8675,
    },
};

pub const L80: Line = Line {
    start: Point { x: 60., y: 92.376 },
    delta: Point {
        x: -50.,
        y: 28.8675,
    },
};
pub const L81: Line = Line {
    start: Point { x: 40., y: 57.735 },
    delta: Point { x: 0., y: -57.735 },
};

pub const L83: Line = Line {
    start: Point { x: 80., y: 57.735 },
    delta: Point { x: 50., y: 28.8675 },
};

pub const L92: Line = Line {
    start: Point { x: 60., y: 11.547 },
    delta: Point { x: 40., y: 23.094 },
};

pub const L96: Line = Line {
    start: Point { x: 80., y: 46.188 },
    delta: Point { x: -40., y: 23.094 },
};

pub const L98: Line = Line {
    start: Point { x: 40., y: 80.829 },
    delta: Point { x: 40., y: 23.094 },
};
pub const L99: Line = Line {
    start: Point { x: 80., y: 80.829 },
    delta: Point { x: 0., y: -46.188 },
};

pub const L102: Line = Line {
    start: Point { x: 60., y: 46.188 },
    delta: Point { x: -40., y: 23.094 },
};
pub const L103: Line = Line {
    start: Point { x: 60., y: 57.735 },
    delta: Point {
        x: -40.,
        y: -23.094,
    },
};

pub const L106: Line = Line {
    start: Point { x: 40., y: 23.094 },
    delta: Point { x: 40., y: -23.094 },
};
pub const L110: Line = Line {
    start: Point { x: 40., y: 46.188 },
    delta: Point { x: 0., y: -46.188 },
};
pub const L116: Line = Line {
    start: Point { x: 60., y: 92.376 },
    delta: Point { x: 40., y: -23.094 },
};
pub const L117: Line = Line {
    start: Point { x: 40., y: 57.735 },
    delta: Point { x: 0., y: 46.188 },
};

pub const L120: Line = Line {
    start: Point { x: 80., y: 57.735 },
    delta: Point {
        x: -40.,
        y: -23.094,
    },
};

pub const L123: Line = Line {
    start: Point { x: 80., y: 23.094 },
    delta: Point { x: 0., y: 46.188 },
};
