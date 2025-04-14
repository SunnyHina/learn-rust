fn main() {
    let x = celsius_2_fahrenheit(100.0);
    let y = fahrenheit_2_celsius(100.0);
    println!("celsius to fahrenheit: {}, fahrenheit to celsius: {}", x, y);

    let z = fibonacci(12);
    println!("fibonacci: {}", z);

    xmas_song()
}

// 摄氏度转华氏度
fn celsius_2_fahrenheit(celsius: f64) -> f64 {
    return 32.0+celsius*1.8;
}

fn fahrenheit_2_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit-32.0)/1.8
}

// 生成第 n 个斐波那契数
// 1、 1、 2、 3、 5、 8、 13、 21、 34、 55、 89、 144、 233、 377、 610
fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }else {
        return fibonacci(n-1) + fibonacci(n-2);
    }
}

// 打印圣诞颂歌 “The Twelve Days of Christmas” 的歌词，并利用歌曲中的重复部分（编写循环）。
// On the first day of Christmas
// My true love sent to me
// A partridge in a pear tree
// On the second day of Christmas
// My true love sent to me
// Two turtle-doves
// And a partridge in a pear tree
// On the third day of Christmas
// My true love sent to me
// Three French hens
// Two turtle-doves
// And a partridge in a pear tree
// On the fourth day of Christmas
// My true love sent to me
// Four calling birds
// Three French hens
// Two turtle-doves
// And a partridge in a pear tree
// On the fifth day of Christmas
// My true love sent to me
// Five golden rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtle-doves
// And a partridge in a pear tree
// On the sixth day of Christmas
// My true love sent to me
// Six geese a laying
// Five golden rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtle-doves
// And a partridge in a pear tree
// On the seventh day of Christmas
// My true love sent to me
// Seven swans a swimming
// Six geese a-laying
// Five golden rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtle-doves
// And a partridge in a pear tree
// On the eighth day of Christmas
// My true love sent to me
// Eight maids a milking
// Seven swans a swimming
// Six geese a-laying
// Five golden rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtle-doves
// And a partridge in a pear tree
// On the ninth day of Christmas
// My true love sent to me
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtle-doves
// And a partridge in a pear tree
// On the tenth day of Christmas
// My true love sent to me
// Ten lords a-leaping
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtle-doves
// And a partridge in a pear tree
// On the 11th day of Christmas
// My true love sent to me
// I sent 11 pipers piping
// Ten lords a-leaping
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtle-doves
// And a partridge in a pear tree
// On the 12th day of Christmas
// My true love sent to me
// 12 drummers drumming
// Eleven pipers piping
// Ten lords a-leaping
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtle-doves
// And a partridge in a pear tree
// And a partridge in a pear tree

fn xmas_song () {
    ()
}