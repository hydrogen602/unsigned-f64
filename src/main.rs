fn main() {
    println!("Hello, world!");
    let p = positive_f64::PositiveF64::new(2.0).unwrap();
    let x = p + positive_f64::PositiveF64::ZERO;
    let y = positive_f64::PositiveF64::square(1.2);
}
