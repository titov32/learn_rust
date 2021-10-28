fn multiply(x: u128) -> u128 {
    let z=x*x*x;
    let y=z*z*z*z*z*z;
    y*y*y
}

fn main() {
    let y = multiply(6);
    println!("Степень числа 6 в квадрате два равно {}", y );
}