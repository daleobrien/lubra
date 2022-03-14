use crate::renders::primatives::{Line, Point};

pub const U: f64 = 120.0;
pub const Q1: f64 = 30.0;
pub const Q3: f64 = 90.0;
pub const M: f64 = 60.0;
pub const Z: f64 = 0.0;

pub const W: f64 = 20.0;

// U * f64::sqrt(3.0) / 2.0;
pub const H: f64 = 103.92304845413263761;

// U * f64::sqrt(3.0) / 6.0;
pub const D: f64 = 34.641016151377545871;

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
pub const L55: Line = Line {
    start: Point { x: 40., y: 103.923 },
    delta: Point { x: 0., y: -103.923 },
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
pub const L58: Line = Line {
    start: Point { x: 80., y: 103.923 },
    delta: Point { x: 0., y: -103.923 },
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
pub const L61: Line = Line {
    start: Point { x: 40., y: 23.094 },
    delta: Point { x: 0., y: -23.094 },
};
pub const L62: Line = Line {
    start: Point { x: 40., y: 23.094 },
    delta: Point { x: -20., y: 11.547 },
};
pub const L63: Line = Line {
    start: Point { x: 80., y: 23.094 },
    delta: Point { x: 0., y: -23.094 },
};
pub const L64: Line = Line {
    start: Point { x: 80., y: 23.094 },
    delta: Point { x: 20., y: 11.547 },
};
pub const L65: Line = Line {
    start: Point { x: 60., y: 57.735 },
    delta: Point { x: -20., y: 11.547 },
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
pub const L68: Line = Line {
    start: Point { x: 40., y: 80.829 },
    delta: Point { x: 0., y: 23.094 },
};
pub const L69: Line = Line {
    start: Point { x: 80., y: 80.829 },
    delta: Point { x: 0., y: 23.094 },
};
pub const L70: Line = Line {
    start: Point { x: 80., y: 80.829 },
    delta: Point { x: 20., y: -11.547 },
};
pub const L71: Line = Line {
    start: Point { x: 60., y: 46.188 },
    delta: Point {
        x: -20.,
        y: -11.547,
    },
};
pub const L72: Line = Line {
    start: Point { x: 60., y: 46.188 },
    delta: Point { x: 20., y: -11.547 },
};
pub const L73: Line = Line {
    start: Point { x: 60., y: 11.547 },
    delta: Point {
        x: 50.,
        y: -28.8675,
    },
};
pub const L74: Line = Line {
    start: Point { x: 60., y: 11.547 },
    delta: Point {
        x: -50.,
        y: -28.8675,
    },
};
pub const L75: Line = Line {
    start: Point { x: 40., y: 46.188 },
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
pub const L78: Line = Line {
    start: Point { x: 80., y: 46.188 },
    delta: Point { x: 0., y: 57.735 },
};
pub const L79: Line = Line {
    start: Point { x: 60., y: 92.376 },
    delta: Point { x: 50., y: 28.8675 },
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
pub const L82: Line = Line {
    start: Point { x: 40., y: 57.735 },
    delta: Point {
        x: -50.,
        y: 28.8675,
    },
};
pub const L83: Line = Line {
    start: Point { x: 80., y: 57.735 },
    delta: Point { x: 50., y: 28.8675 },
};
pub const L84: Line = Line {
    start: Point { x: 80., y: 57.735 },
    delta: Point { x: 0., y: -57.735 },
};
pub const L85: Line = Line {
    start: Point { x: 40., y: 57.735 },
    delta: Point { x: 40., y: -23.094 },
};
pub const L86: Line = Line {
    start: Point { x: 40., y: 57.735 },
    delta: Point { x: 0., y: 46.188 },
};
pub const L87: Line = Line {
    start: Point { x: 60., y: 92.376 },
    delta: Point { x: 40., y: -23.094 },
};
pub const L88: Line = Line {
    start: Point { x: 60., y: 92.376 },
    delta: Point {
        x: -40.,
        y: -23.094,
    },
};
pub const L89: Line = Line {
    start: Point { x: 80., y: 57.735 },
    delta: Point {
        x: -40.,
        y: -23.094,
    },
};
pub const L90: Line = Line {
    start: Point { x: 80., y: 57.735 },
    delta: Point { x: 0., y: 46.188 },
};
pub const L91: Line = Line {
    start: Point { x: 60., y: 11.547 },
    delta: Point { x: -40., y: 23.094 },
};
pub const L92: Line = Line {
    start: Point { x: 60., y: 11.547 },
    delta: Point { x: 40., y: 23.094 },
};
pub const L93: Line = Line {
    start: Point { x: 40., y: 46.188 },
    delta: Point { x: 0., y: -46.188 },
};
pub const L94: Line = Line {
    start: Point { x: 40., y: 46.188 },
    delta: Point { x: 40., y: 23.094 },
};
pub const L95: Line = Line {
    start: Point { x: 80., y: 46.188 },
    delta: Point { x: 0., y: -46.188 },
};
pub const L96: Line = Line {
    start: Point { x: 80., y: 46.188 },
    delta: Point { x: -40., y: 23.094 },
};
pub const L97: Line = Line {
    start: Point { x: 40., y: 80.829 },
    delta: Point { x: 0., y: -46.188 },
};
pub const L98: Line = Line {
    start: Point { x: 40., y: 80.829 },
    delta: Point { x: 40., y: 23.094 },
};
pub const L99: Line = Line {
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
pub const L108: Line = Line {
    start: Point { x: 80., y: 23.094 },
    delta: Point { x: 20., y: 11.547 },
};
pub const L109: Line = Line {
    start: Point { x: 40., y: 46.188 },
    delta: Point { x: 40., y: 23.094 },
};
pub const L110: Line = Line {
    start: Point { x: 40., y: 46.188 },
    delta: Point { x: 0., y: -46.188 },
};
pub const L111: Line = Line {
    start: Point { x: 60., y: 11.547 },
    delta: Point { x: 40., y: 23.094 },
};
pub const L112: Line = Line {
    start: Point { x: 60., y: 11.547 },
    delta: Point { x: -40., y: 23.094 },
};
pub const L113: Line = Line {
    start: Point { x: 80., y: 46.188 },
    delta: Point { x: -40., y: 23.094 },
};
pub const L114: Line = Line {
    start: Point { x: 80., y: 46.188 },
    delta: Point { x: 0., y: -46.188 },
};
pub const L115: Line = Line {
    start: Point { x: 60., y: 92.376 },
    delta: Point {
        x: -40.,
        y: -23.094,
    },
};
pub const L116: Line = Line {
    start: Point { x: 60., y: 92.376 },
    delta: Point { x: 40., y: -23.094 },
};
pub const L117: Line = Line {
    start: Point { x: 40., y: 57.735 },
    delta: Point { x: 0., y: 46.188 },
};
pub const L118: Line = Line {
    start: Point { x: 40., y: 57.735 },
    delta: Point { x: 40., y: -23.094 },
};
pub const L119: Line = Line {
    start: Point { x: 80., y: 57.735 },
    delta: Point { x: 0., y: 46.188 },
};
pub const L120: Line = Line {
    start: Point { x: 80., y: 57.735 },
    delta: Point {
        x: -40.,
        y: -23.094,
    },
};
pub const L121: Line = Line {
    start: Point { x: 40., y: 23.094 },
    delta: Point { x: 0., y: 46.188 },
};
pub const L122: Line = Line {
    start: Point { x: 40., y: 23.094 },
    delta: Point { x: 40., y: -23.094 },
};
pub const L123: Line = Line {
    start: Point { x: 80., y: 23.094 },
    delta: Point { x: 0., y: 46.188 },
};
pub const L124: Line = Line {
    start: Point { x: 80., y: 23.094 },
    delta: Point {
        x: -40.,
        y: -23.094,
    },
};
pub const L125: Line = Line {
    start: Point { x: 60., y: 57.735 },
    delta: Point { x: 40., y: -23.094 },
};
pub const L126: Line = Line {
    start: Point { x: 60., y: 57.735 },
    delta: Point {
        x: -40.,
        y: -23.094,
    },
};
pub const L127: Line = Line {
    start: Point { x: 60., y: 46.188 },
    delta: Point { x: -40., y: 23.094 },
};
pub const L128: Line = Line {
    start: Point { x: 60., y: 46.188 },
    delta: Point { x: 40., y: 23.094 },
};
pub const L129: Line = Line {
    start: Point { x: 40., y: 80.829 },
    delta: Point { x: 0., y: -46.188 },
};
pub const L130: Line = Line {
    start: Point { x: 40., y: 80.829 },
    delta: Point { x: 40., y: 23.094 },
};
pub const L131: Line = Line {
    start: Point { x: 80., y: 80.829 },
    delta: Point { x: -40., y: 23.094 },
};
