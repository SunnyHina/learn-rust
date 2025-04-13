fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // shadowing属于新创建一个变量,可以随便更改
    let spaces = "   ";
    let spaces = spaces.len();

    // 声明为可变变量不能改变变量的类型
    let mut spaces = "   ";
    spaces = spaces.len();

}