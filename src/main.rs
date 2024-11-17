fn main() {
    let x_to = 3.0;
    let from_x = 1.0;
    let angle: f64 = -59.5694837374164;
    let x_component = x_to - from_x;
    let y_component = x_component * f64::tan(angle.to_radians());
    println!("{y_component}");
}
