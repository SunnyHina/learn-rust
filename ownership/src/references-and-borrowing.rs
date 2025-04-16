//references-and-borrowing

// 在任意给定时间(每一行代码?)，要么只能有一个可变引用，要么只能有多个不可变引用。
// 引用必须总是有效的。

fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1); //像是指针

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);
    println!("{}", &&s1);

    // 不允许这样声明两个可变引用(除非不用他们)
    // 防止了同时操作一个资源,避免数据竞争
    // let r1 = &mut s1;
    // let r2 = &mut s1;
    // println!("{}", r1);

    let r1 = &mut s1;
    println!("{}", r1);
    let r2 = &mut s1;
    println!("{}", r2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}