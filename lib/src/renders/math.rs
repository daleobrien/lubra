pub fn polar(x: f64, y: f64, radius: f64, angle: f64) -> (f64, f64) {
    let (a, b) = f64::sin_cos(angle);
    return (x + (radius * b), y + (radius * a));
}
