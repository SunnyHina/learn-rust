fn main() {
    another_function(-5);
    print_labeled_measurement(5, 'h');
    statement_test();
    let x = five();
    println!("five:{x}");
    let y = plus_one(x);
    println!("x plus 1 = {y}")
}

fn another_function(x: i8) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn statement_test() {
    let y = {
        let x = 3;
        x + 1
    };
    // `x + 1`没有以分号结尾,是一个表达式,是返回值?
    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}