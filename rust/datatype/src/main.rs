fn main() {
    let x = 2.1;
    let y: f32 = 3.1;

    let z: f64 = (x + y).into();

    println!("z={}", z);

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7;
    let remainder = 43 % 5;

    let s = 1;
    let s = s + 1;

    let c : char = 'a';

    let tup = (1, 2.2, "hello");

    let (x, y, z) = tup;

    let a = [1, 2, 3, 4];

    let b = ["Jan", "Feb"];
    println!("{}", a[0]);
    println!("{}", b[0]);

}
