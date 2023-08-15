const SQRT_3: f64 = 3f64.sqrt();
const FRAC_2: f64 = 0.5f64;
const FRAC_SQRT_3: f64 = 0.57735026918962576450914878050195746f64;
const FRAC_2_SQRT_3: f64 = 0.28867513459481288225457439025097873f64;
const FRAC_3_SQRT_3: f64 = 0.19245008972987525483638292683398582f64;

const P1: (f64, f64) = (1 / 2, SQRT_3 / 2);
const P2: (f64, f64) = (0, 0);
const P3: (f64, f64) = (1, 0);

const P41: (f64, f64) = (1 / 3, 1 / SQRT_3);
const P42: (f64, f64) = (1 / 6, FRAC_2_SQRT_3);
const P43: (f64, f64) = (1 / 3, FRAC_3_SQRT_3);
const P44: (f64, f64) = (0, FRAC_SQRT_3);

const P51: (f64, f64) = (2 / 3, FRAC_SQRT_3);
const P52: (f64, f64) = (2 / 3, FRAC_3_SQRT_3);
const P53: (f64, f64) = (6 / 6, FRAC_2_SQRT_3);
const P55: (f64, f64) = (1, FRAC_SQRT_3);

const P61: (f64, f64) = (0, FRAC_SQRT_3 / 6);
const P62: (f64, f64) = (1 / 3, 0);
const P63: (f64, f64) = (2 / 3, 0);
const P66: (f64, f64) = (0, FRAC_SQRT_3);

/*

fn generate_L1(coords: (f64, f64), size: f64) -> String {
    let (x, y) = coords;
    let (x, y) = (x * size, y * size);
    format!(
        "M {} {} l {} {} l {} {} l {} {} z m",
        x,
        y,
        size * P1.0,
        size * P1.1,
        size * P2.0,
        size * P2.1,
        size * P3.0,
        size * P3.1
    )
}


fn gererate_L2(coords: (f64, f64)) -> String {

    write!("{}{}", Line { coord, P1, P2}, Line { coord, P2, P3 })
    )

}
 */
