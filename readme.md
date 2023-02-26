# UnsignedF64

A library to add the equivalent of unsigned ints to floats by making a `UnsignedF64` that can't be negative.

## How To

Create new unsigned floats using `UnsignedF64::new`. This will check if the `f64` is negative or not and return an `Option`. Many `f64` methods are implemented on `UnsignedF64` so that numbers don't have to be rechecked when non-negativeness is guaranteed, e.g. the square root of an `UnsignedF64` is guaranteed to be non-negative, so `sqrt` returns `UnsignedF64`.

```Rust
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
```

## ToDo

- Implement the methods where I'm not sure if they can return negative numbers or not
- Implement `std::ops::Rem` for `UnsignedF64`
- Implement traits for `&UnsignedF64`
- Implement serde's Serialize & Deserialize (using features?)