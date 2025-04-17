struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 元组结构体（tuple structs）
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 类单元结构体（unit-like structs）
struct AlwaysEqual;

fn main() {
    let user1 = User{
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("{}", user1.username);
    let user2 = User {
        active: user1.active,
        username: String::from("anotherusername567"),
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("{}", user2.username);
    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };
    // 现在不能使用user2了
    // 如果只使用结构体中实现了Copy trait 的类型创建新的结构体,那么原来的结构体还可以使用
    // println!("{}", user2.username);
    println!("{}", user3.username);
    println!("{}", user3.email);
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    let subject = AlwaysEqual;
}


// struct User {
//     active: bool,
// 如果在结构体中使用引用,需要声明生命周期
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: "someusername123",
//         email: "someone@example.com",
//         sign_in_count: 1,
//     };
// }