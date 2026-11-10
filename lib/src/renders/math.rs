pub fn polar(x: f64, y: f64, radius: f64, angle: f64) -> (f64, f64) {
    let (a, b) = f64::sin_cos(angle);
    return (x + (radius * b), y + (radius * a));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_polar_at_origin() {
        let (x, y) = polar(0.0, 0.0, 10.0, 0.0);
        assert!((x - 10.0).abs() < 1e-10);
        assert!(y.abs() < 1e-10);
    }

    #[test]
    fn test_polar_90_degrees() {
        let (x, y) = polar(0.0, 0.0, 10.0, PI / 2.0);
        assert!(x.abs() < 1e-10);
        assert!((y - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_polar_180_degrees() {
        let (x, y) = polar(0.0, 0.0, 10.0, PI);
        assert!((x + 10.0).abs() < 1e-10);
        assert!(y.abs() < 1e-10);
    }

    #[test]
    fn test_polar_270_degrees() {
        let (x, y) = polar(0.0, 0.0, 10.0, 3.0 * PI / 2.0);
        assert!(x.abs() < 1e-10);
        assert!((y + 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_polar_with_offset() {
        let (x, y) = polar(5.0, 5.0, 10.0, 0.0);
        assert!((x - 15.0).abs() < 1e-10);
        assert!((y - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_polar_45_degrees() {
        let (x, y) = polar(0.0, 0.0, 10.0, PI / 4.0);
        let expected = 10.0 / f64::sqrt(2.0);
        assert!((x - expected).abs() < 1e-10);
        assert!((y - expected).abs() < 1e-10);
    }
}
