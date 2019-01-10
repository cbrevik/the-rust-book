fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("Max points: {}", MAX_POINTS);

    let x = 2.0; // f64

    let _y: f32 = 3.0; // f32

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup2 = (500, 6.4, 1);

    let (_, y, _) = tup;
    println!("The value of y is: {}", y);

    let a = [1, 2, 3, 4, 5];
}
