fn main() {
    // 数组中的每个元素的类型必须相同

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let a = [3, 3, 3, 3, 3];
    let b = [3; 5];

    let first = a[0];
    let second = a[1];
    println!("The first value is: {first}");
    println!("The second value is: {second}");
    let some = a[9];
}