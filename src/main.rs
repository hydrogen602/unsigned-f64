use unsigned_f64::UnsignedF64;

fn main() {
    // An example of using the UnsignedF64 type.

    let point1 = (3., 4.);
    let point2 = (5., 12.);

    fn distance(p1: (f64, f64), p2: (f64, f64)) -> UnsignedF64 {
        let x = p1.0 - p2.0;
        let y = p1.1 - p2.1;
        (UnsignedF64::square(x) + UnsignedF64::square(y)).sqrt()
    }

    let d = distance(point1, point2);

    println!(
        "The distance between {:?} and {:?} is {}",
        point1, point2, d
    );
}
