fn main() {
    // 元组 长度不可变
    let mut tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    println!("The value of x is: {}", tup.0);
    tup.0 = 100;
    println!("The value of x is: {}", tup.0);
}