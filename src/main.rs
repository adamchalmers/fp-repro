fn main() {
    let angle: f64 = -59.5694837374164;
    let angle_r = angle.to_radians();
    println!("{angle_r}");
    let y_component = f64::tan(angle_r);
    println!("{y_component}");
}
