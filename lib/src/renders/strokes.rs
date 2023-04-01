use crate::renders::primatives::{Line, Point};

pub const SMALL_ANGLE: f64 = 0.2709185204562202f64;

pub const U: f64 = 120.0;
pub const Q1: f64 = 30.0;
pub const Q3: f64 = 90.0;
pub const M: f64 = 60.0;
pub const Z: f64 = 0.0;

pub const W: f64 = 20.0;

// U * f64::sqrt(3.0) / 2.0;
pub const H: f64 = 103.92304845413263761f64;

// U * f64::sqrt(3.0) / 6.0;
// pub const D: f64 = 34.641016151377545871;

pub const L001: Line = Line {
    start: Point { x: 40., y: 0. },
    delta: Point {
        x: -30.,
        y: -17.3205,
    },
};
pub const L002: Line = Line {
    start: Point {
        x: -10.,
        y: 17.3205,
    },
    delta: Point { x: 30., y: 17.3205 },
};
pub const L003: Line = Line {
    start: Point { x: 80., y: 69.282 },
    delta: Point {
        x: -30.,
        y: -17.3205,
    },
};
pub const L004: Line = Line {
    start: Point { x: 100., y: 34.641 },
    delta: Point {
        x: -30.,
        y: -17.3205,
    },
};
pub const L005: Line = Line {
    start: Point { x: 40., y: 0. },
    delta: Point { x: 0., y: 34.641 },
};
pub const L006: Line = Line {
    start: Point { x: 80., y: 0. },
    delta: Point { x: 0., y: 34.641 },
};
pub const L007: Line = Line {
    start: Point { x: 40., y: 69.282 },
    delta: Point { x: 0., y: 34.641 },
};
pub const L008: Line = Line {
    start: Point { x: 80., y: 103.923 },
    delta: Point { x: 0., y: -34.641 },
};
pub const L009: Line = Line {
    start: Point { x: 40., y: 69.282 },
    delta: Point {
        x: 30.,
        y: -17.3205,
    },
};
pub const L010: Line = Line {
    start: Point { x: 20., y: 34.641 },
    delta: Point {
        x: 30.,
        y: -17.3205,
    },
};
pub const L011: Line = Line {
    start: Point { x: 100., y: 34.641 },
    delta: Point {
        x: 30.,
        y: -17.3205,
    },
};
pub const L012: Line = Line {
    start: Point {
        x: 110.,
        y: -17.3205,
    },
    delta: Point {
        x: -30.,
        y: 17.3205,
    },
};
pub const L013: Line = Line {
    start: Point { x: 20., y: 34.641 },
    delta: Point {
        x: 22.918,
        y: -13.2317,
    },
};
pub const L014: Line = Line {
    start: Point { x: 40., y: 69.282 },
    delta: Point {
        x: 22.918,
        y: -13.2317,
    },
};
pub const L015: Line = Line {
    start: Point { x: 80., y: 69.282 },
    delta: Point {
        x: -22.918,
        y: -13.2317,
    },
};
pub const L016: Line = Line {
    start: Point { x: 100., y: 34.641 },
    delta: Point {
        x: -22.918,
        y: -13.2317,
    },
};
pub const L017: Line = Line {
    start: Point { x: 80., y: 0. },
    delta: Point { x: 0., y: 26.4634 },
};
pub const L018: Line = Line {
    start: Point { x: 40., y: 0. },
    delta: Point { x: 0., y: 26.4634 },
};
pub const L019: Line = Line {
    start: Point { x: 20., y: 34.641 },
    delta: Point {
        x: 37.082,
        y: 21.4093,
    },
};
pub const L020: Line = Line {
    start: Point { x: 40., y: 0. },
    delta: Point {
        x: 37.082,
        y: 21.4093,
    },
};
pub const L021: Line = Line {
    start: Point { x: 40., y: 26.4634 },
    delta: Point { x: 0., y: 42.8187 },
};
pub const L022: Line = Line {
    start: Point { x: 80., y: 26.4634 },
    delta: Point { x: 0., y: 42.8187 },
};
pub const L023: Line = Line {
    start: Point { x: 80., y: 0. },
    delta: Point {
        x: -37.082,
        y: 21.4093,
    },
};
pub const L024: Line = Line {
    start: Point { x: 100., y: 34.641 },
    delta: Point {
        x: -37.082,
        y: 21.4093,
    },
};
pub const L025: Line = Line {
    start: Point { x: 40., y: 103.923 },
    delta: Point {
        x: -30.,
        y: 17.3205,
    },
};
pub const L026: Line = Line {
    start: Point {
        x: -10.,
        y: 86.6025,
    },
    delta: Point {
        x: 30.,
        y: -17.3205,
    },
};
pub const L027: Line = Line {
    start: Point { x: 80., y: 34.641 },
    delta: Point {
        x: -30.,
        y: 17.3205,
    },
};
pub const L028: Line = Line {
    start: Point { x: 100., y: 69.282 },
    delta: Point {
        x: -30.,
        y: 17.3205,
    },
};
pub const L033: Line = Line {
    start: Point { x: 40., y: 34.641 },
    delta: Point { x: 30., y: 17.3205 },
};
pub const L034: Line = Line {
    start: Point { x: 20., y: 69.282 },
    delta: Point { x: 30., y: 17.3205 },
};
pub const L035: Line = Line {
    start: Point { x: 100., y: 69.282 },
    delta: Point { x: 30., y: 17.3205 },
};
pub const L036: Line = Line {
    start: Point {
        x: 110.,
        y: 121.244,
    },
    delta: Point {
        x: -30.,
        y: -17.3205,
    },
};
pub const L037: Line = Line {
    start: Point { x: 20., y: 69.282 },
    delta: Point {
        x: 22.918,
        y: 13.2317,
    },
};
pub const L038: Line = Line {
    start: Point { x: 40., y: 34.641 },
    delta: Point {
        x: 22.918,
        y: 13.2317,
    },
};
pub const L039: Line = Line {
    start: Point { x: 80., y: 34.641 },
    delta: Point {
        x: -22.918,
        y: 13.2317,
    },
};
pub const L040: Line = Line {
    start: Point { x: 100., y: 69.282 },
    delta: Point {
        x: -22.918,
        y: 13.2317,
    },
};
pub const L041: Line = Line {
    start: Point { x: 80., y: 103.923 },
    delta: Point { x: 0., y: -26.4634 },
};
pub const L042: Line = Line {
    start: Point { x: 40., y: 103.923 },
    delta: Point { x: 0., y: -26.4634 },
};
pub const L043: Line = Line {
    start: Point { x: 20., y: 69.282 },
    delta: Point {
        x: 37.082,
        y: -21.4093,
    },
};
pub const L044: Line = Line {
    start: Point { x: 40., y: 103.923 },
    delta: Point {
        x: 37.082,
        y: -21.4093,
    },
};
pub const L045: Line = Line {
    start: Point { x: 40., y: 77.4597 },
    delta: Point { x: 0., y: -42.8187 },
};
pub const L046: Line = Line {
    start: Point { x: 80., y: 77.4597 },
    delta: Point { x: 0., y: -42.8187 },
};
pub const L047: Line = Line {
    start: Point { x: 80., y: 103.923 },
    delta: Point {
        x: -37.082,
        y: -21.4093,
    },
};
pub const L048: Line = Line {
    start: Point { x: 100., y: 69.282 },
    delta: Point {
        x: -37.082,
        y: -21.4093,
    },
};
pub const L049: Line = Line {
    start: Point { x: 40., y: 0. },
    delta: Point { x: 0., y: 103.923 },
};
pub const L050: Line = Line {
    start: Point { x: 40., y: 69.282 },
    delta: Point {
        x: 90.,
        y: -51.9615,
    },
};
pub const L051: Line = Line {
    start: Point { x: 80., y: 69.282 },
    delta: Point {
        x: -90.,
        y: -51.9615,
    },
};
pub const L052: Line = Line {
    start: Point { x: 80., y: 0. },
    delta: Point { x: 0., y: 103.923 },
};
pub const L053: Line = Line {
    start: Point { x: 20., y: 34.641 },
    delta: Point {
        x: 90.,
        y: -51.9615,
    },
};
pub const L054: Line = Line {
    start: Point { x: 100., y: 34.641 },
    delta: Point {
        x: -90.,
        y: -51.9615,
    },
};
pub const L056: Line = Line {
    start: Point { x: 40., y: 34.641 },
    delta: Point { x: 90., y: 51.9615 },
};
pub const L057: Line = Line {
    start: Point { x: 80., y: 34.641 },
    delta: Point {
        x: -90.,
        y: 51.9615,
    },
};
pub const L059: Line = Line {
    start: Point { x: 20., y: 69.282 },
    delta: Point { x: 90., y: 51.9615 },
};
pub const L060: Line = Line {
    start: Point { x: 100., y: 69.282 },
    delta: Point {
        x: -90.,
        y: 51.9615,
    },
};
pub const L061: Line = Line {
    start: Point { x: 40., y: 23.094 },
    delta: Point { x: 0., y: -23.094 },
};
pub const L062: Line = Line {
    start: Point { x: 40., y: 23.094 },
    delta: Point { x: -20., y: 11.547 },
};
pub const L063: Line = Line {
    start: Point { x: 80., y: 23.094 },
    delta: Point { x: 0., y: -23.094 },
};
pub const L064: Line = Line {
    start: Point { x: 80., y: 23.094 },
    delta: Point { x: 20., y: 11.547 },
};
pub const L065: Line = Line {
    start: Point { x: 60., y: 57.735 },
    delta: Point { x: -20., y: 11.547 },
};
pub const L066: Line = Line {
    start: Point { x: 60., y: 57.735 },
    delta: Point { x: 20., y: 11.547 },
};
pub const L067: Line = Line {
    start: Point { x: 40., y: 80.829 },
    delta: Point {
        x: -20.,
        y: -11.547,
    },
};
pub const L068: Line = Line {
    start: Point { x: 40., y: 80.829 },
    delta: Point { x: 0., y: 23.094 },
};
pub const L069: Line = Line {
    start: Point { x: 80., y: 80.829 },
    delta: Point { x: 0., y: 23.094 },
};
pub const L070: Line = Line {
    start: Point { x: 80., y: 80.829 },
    delta: Point { x: 20., y: -11.547 },
};
pub const L071: Line = Line {
    start: Point { x: 60., y: 46.188 },
    delta: Point {
        x: -20.,
        y: -11.547,
    },
};
pub const L072: Line = Line {
    start: Point { x: 60., y: 46.188 },
    delta: Point { x: 20., y: -11.547 },
};
pub const L073: Line = Line {
    start: Point { x: 60., y: 11.547 },
    delta: Point {
        x: 50.,
        y: -28.8675,
    },
};
pub const L074: Line = Line {
    start: Point { x: 60., y: 11.547 },
    delta: Point {
        x: -50.,
        y: -28.8675,
    },
};
pub const L075: Line = Line {
    start: Point { x: 40., y: 46.188 },
    delta: Point {
        x: -50.,
        y: -28.8675,
    },
};
pub const L076: Line = Line {
    start: Point { x: 40., y: 46.188 },
    delta: Point { x: 0., y: 57.735 },
};
pub const L077: Line = Line {
    start: Point { x: 80., y: 46.188 },
    delta: Point {
        x: 50.,
        y: -28.8675,
    },
};
pub const L078: Line = Line {
    start: Point { x: 80., y: 46.188 },
    delta: Point { x: 0., y: 57.735 },
};
pub const L079: Line = Line {
    start: Point { x: 60., y: 92.376 },
    delta: Point { x: 50., y: 28.8675 },
};
pub const L080: Line = Line {
    start: Point { x: 60., y: 92.376 },
    delta: Point {
        x: -50.,
        y: 28.8675,
    },
};
pub const L081: Line = Line {
    start: Point { x: 40., y: 57.735 },
    delta: Point { x: 0., y: -57.735 },
};
pub const L082: Line = Line {
    start: Point { x: 40., y: 57.735 },
    delta: Point {
        x: -50.,
        y: 28.8675,
    },
};
pub const L083: Line = Line {
    start: Point { x: 80., y: 57.735 },
    delta: Point { x: 50., y: 28.8675 },
};
pub const L084: Line = Line {
    start: Point { x: 80., y: 57.735 },
    delta: Point { x: 0., y: -57.735 },
};
pub const L085: Line = Line {
    start: Point { x: 40., y: 57.735 },
    delta: Point { x: 40., y: -23.094 },
};
pub const L086: Line = Line {
    start: Point { x: 40., y: 57.735 },
    delta: Point { x: 0., y: 46.188 },
};
pub const L087: Line = Line {
    start: Point { x: 60., y: 92.376 },
    delta: Point { x: 40., y: -23.094 },
};
pub const L088: Line = Line {
    start: Point { x: 60., y: 92.376 },
    delta: Point {
        x: -40.,
        y: -23.094,
    },
};
pub const L089: Line = Line {
    start: Point { x: 80., y: 57.735 },
    delta: Point {
        x: -40.,
        y: -23.094,
    },
};
pub const L090: Line = Line {
    start: Point { x: 80., y: 57.735 },
    delta: Point { x: 0., y: 46.188 },
};
pub const L091: Line = Line {
    start: Point { x: 60., y: 11.547 },
    delta: Point { x: -40., y: 23.094 },
};
pub const L092: Line = Line {
    start: Point { x: 60., y: 11.547 },
    delta: Point { x: 40., y: 23.094 },
};
pub const L093: Line = Line {
    start: Point { x: 40., y: 46.188 },
    delta: Point { x: 0., y: -46.188 },
};
pub const L094: Line = Line {
    start: Point { x: 40., y: 46.188 },
    delta: Point { x: 40., y: 23.094 },
};
pub const L095: Line = Line {
    start: Point { x: 80., y: 46.188 },
    delta: Point { x: 0., y: -46.188 },
};
pub const L096: Line = Line {
    start: Point { x: 80., y: 46.188 },
    delta: Point { x: -40., y: 23.094 },
};
pub const L097: Line = Line {
    start: Point { x: 40., y: 80.829 },
    delta: Point { x: 0., y: -46.188 },
};
pub const L098: Line = Line {
    start: Point { x: 40., y: 80.829 },
    delta: Point { x: 40., y: 23.094 },
};
pub const L099: Line = Line {
    start: Point { x: 80., y: 80.829 },
    delta: Point { x: 0., y: -46.188 },
};
pub const L100: Line = Line {
    start: Point { x: 80., y: 80.829 },
    delta: Point { x: -40., y: 23.094 },
};
pub const L101: Line = Line {
    start: Point { x: 60., y: 46.188 },
    delta: Point { x: 40., y: 23.094 },
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
pub const L104: Line = Line {
    start: Point { x: 60., y: 57.735 },
    delta: Point { x: 40., y: -23.094 },
};
pub const L105: Line = Line {
    start: Point { x: 40., y: 23.094 },
    delta: Point { x: 0., y: 46.188 },
};
pub const L106: Line = Line {
    start: Point { x: 40., y: 23.094 },
    delta: Point { x: 40., y: -23.094 },
};
pub const L107: Line = Line {
    start: Point { x: 80., y: 23.094 },
    delta: Point {
        x: -40.,
        y: -23.094,
    },
};
pub const L121: Line = Line {
    start: Point { x: 80., y: 23.094 },
    delta: Point { x: 0., y: 46.188 },
};
