const SQRT_3: f64 = 1.732050807568877293527446341505872f64;

const P14: (f64, f64) = (1f64 / 3f64, SQRT_3 / 2f64);
const P11: (f64, f64) = (1f64 / 2f64, SQRT_3 / 2f64);
const P15: (f64, f64) = (2f64 / 3f64, SQRT_3 / 2f64);

const P21: (f64, f64) = (-1f64 / 12f64, 1f64 / SQRT_3 / 4f64);
const P22: (f64, f64) = (0f64, 0f64);
const P23: (f64, f64) = (1f64 / 12f64, -1f64 / SQRT_3 / 4f64);

const P35: (f64, f64) = (13f64 / 12f64, 1f64 / SQRT_3 / 4f64);
const P33: (f64, f64) = (1f64, 0f64);
const P36: (f64, f64) = (11f64 / 12f64, -1f64 / SQRT_3 / 4f64);

const P41: (f64, f64) = (1f64 / 3f64, 1f64 / SQRT_3);
const P42: (f64, f64) = (1f64 / 6f64, 1f64 / SQRT_3 / 2f64);
const P40: (f64, f64) = (1f64 / 4f64, SQRT_3 / 4f64);
const P44: (f64, f64) = (0f64, 1f64 / SQRT_3);

const P51: (f64, f64) = (2f64 / 3f64, 1f64 / SQRT_3);
const P50: (f64, f64) = (3f64 / 4f64, SQRT_3 / 4f64);
const P53: (f64, f64) = (6f64 / 6f64, 1f64 / SQRT_3 / 2f64);
const P55: (f64, f64) = (1f64, 1f64 / SQRT_3);

const P60: (f64, f64) = (0f64, 1f64 / 2f64);
const P62: (f64, f64) = (1f64 / 3f64, 0f64);
const P63: (f64, f64) = (2f64 / 3f64, 0f64);
const P66: (f64, f64) = (0f64, 1f64 / SQRT_3);

const SCALE: f64 = 120f64;

fn diff_4(
    a: (f64, f64),
    b: (f64, f64),
    c: (f64, f64),
    d: (f64, f64),
) -> (f64, f64, f64, f64, f64, f64) {
    // Convert 4 points in a series of 3 differences
    return (
        SCALE * (b.0 - a.0),
        SCALE * (b.1 - a.1),
        SCALE * (c.0 - b.0),
        SCALE * (c.1 - b.1),
        SCALE * (d.0 - c.0),
        SCALE * (d.1 - c.1),
    );
}

fn wrap_format_with_function(svg: String) -> String {
    format!("{}", svg)
}

fn generate_L1_fn() -> String {
    let (a, b, c, d, e, f) = diff_4(P14, P62, P63, P15);
    return format!(
        "M {{x}} {{y}} l {:.2} {:.2} m {:.2} {:.2} l {:.2} {:.2} z ",
        a, b, c, d, e, f
    );
}

pub fn generate_all_shapes() {
    generate_L1_fn();
}
