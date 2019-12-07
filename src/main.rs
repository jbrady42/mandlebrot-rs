use num::complex::Complex;

fn main() {
    println!("Hello, world!");
    let complex_integer: Complex<u128> = num::complex::Complex::new(10, 20);
    let complex_float = num::complex::Complex::new(10.1, 20.1);

    println!("Complex integer: {}", complex_integer);
    println!("Complex float: {}", complex_float);
}
