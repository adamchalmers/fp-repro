fn main() {
    let x_to = 3.0;
    let from_x = 1.0;
    let angle: f64 = -59.5694837374164;
    let angle_r = angle.to_radians();
    let x_component = x_to - from_x;
    println!("{angle_r}");
    let y_component = x_component * f64::tan(angle_r);
    println!("{y_component}");
}
