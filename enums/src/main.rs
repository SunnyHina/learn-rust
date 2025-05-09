// // 会自动识别数据类型吗?
// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr_s {
//     kind: IpAddrKind,
//     address: String,
// }



// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;

//     // 注意枚举的成员位于其标识符的命名空间中，并使用两个冒号分开。这么设计的益处是现在 IpAddrKind::V4 和 IpAddrKind::V6 都是 IpAddrKind 类型的。
//     // let four_t = IpAddrKind.V4;  // 不可以这样调用
//     // println!("{}", four==four_t)

//     enum IpAddr {
//         V4(u8, u8, u8, u8),
//         V6(String),
//     }

//     let home = IpAddr::V4(127, 0, 0, 1);

//     let loopback = IpAddr::V6(String::from("::1"));

//     let home = IpAddr_s {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };
    
//     let loopback = IpAddr_s {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };

// }

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
