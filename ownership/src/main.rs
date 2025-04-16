// 编写一个函数，该函数接收一个用空格分隔单词的字符串，并返回在该字符串中找到的第一个单词。如果函数在该字符串中并未找到空格，则整个字符串就是一个单词，所以应该返回整个字符串。

fn main() {
    let s = String::from("Hello World!");

    let first_word_i = first_word_index(&s);
    // 字符串 slice（string slice）是 String 中一部分值的引用
    // 切片也是引用,一定有&
    println!("{}", &s[..first_word_i]);
    println!("{}", first_word(&s))
}

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s
}