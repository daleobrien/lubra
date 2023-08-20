/* This file is generated. */
use std::collections::HashMap;

pub const U: f64 = 120f64;
pub const M: f64 = 60f64;
// U * f64::sqrt(3.0) / 2.0;
pub const H: f64 = 103.92304845413263f64;

pub struct Fragment {
    strokes: HashMap<u8, String>,
    fills: HashMap<u8, String>,
}

impl Fragment {

    pub fn get(&self, key: &u8) -> [Option<&String>; 2] {
        return [self.strokes.get(key), self.fills.get(key)];
    }

    pub fn new() -> Fragment {
        let mut strokes:HashMap<u8, String> = HashMap::new();
        let mut fills:HashMap<u8, String> = HashMap::new();


        strokes.insert(0b010000, "M 102  31.1769A 24  24 0 0 0 78  72.7461".to_string());
        strokes.insert(0b010010, "M 132  20.7846l-90  51.9615l-24 -41.5692l 90 -51.9615z".to_string());
        strokes.insert(0b011000, "M  42  72.7461 A  36  36 0 0 1  78  72.7461 L  102  31.1769 A  84  84 0 0 0  18  31.1769 z".to_string());
        strokes.insert(0b100001, "M 36  103.923l 0 -103.923l 48  0l 0  103.923z".to_string());
        strokes.insert(0b000001, "M  84  103.923 A  24  24 0 0 0  36  103.923".to_string());
        strokes.insert(0b100000, "M  36 -0 A  24  24 0 0 0  84  0".to_string());
        strokes.insert(0b001100, "M 12 -20.7846l 90  51.9615l-24  41.5692l-90 -51.9615z".to_string());
        strokes.insert(0b000100, "M-12  20.7846A 24  24 0 0 0 12 -20.7846".to_string());
        strokes.insert(0b010100, "M  102  31.1769 A  36  36 0 0 1   84 -0 L  36 -0 A  84  84 0 0 0   78  72.7461 z ".to_string());
        strokes.insert(0b000010, "M  108 -20.7846 A  24  24 0 0 0  132  20.7846".to_string());
        strokes.insert(0b001000, "M  42  72.7461 A  24  24 0 0 0  18  31.1769".to_string());
        strokes.insert(0b110000, "M  36  0 A  36  36 0 0 1  18  31.1769 L  42  72.7461 A  84  84 0 0 0   84  0 z".to_string());
        fills.insert(0b110000, "M  36  0 A  36  36 0 0 1  18  31.1769 M  42  72.7461 A  84  84 0 0 0  84  0".to_string());
        fills.insert(0b010010, "M 132  20.7846l-90  51.9615m-24 -41.5692l 90 -51.9615z".to_string());
        fills.insert(0b011000, "M  42  72.7461 A  36  36 0 0 1  78  72.7461 M  102  31.1769 A  84  84 0 0 0  18  31.1769".to_string());
        fills.insert(0b100001, "M 36  103.923l 0 -103.923m 48  0l 0  103.923z".to_string());
        fills.insert(0b001100, "M 12 -20.7846l 90  51.9615m-24  41.5692l-90 -51.9615z".to_string());
        fills.insert(0b010100, "M  102  31.1769 A  36  36 0 0 1   84 -0 M  36 -0 A  84  84 0 0 0   78  72.7461 ".to_string());

        return Fragment {
             strokes: strokes,
             fills: fills,
        };
    }
}
