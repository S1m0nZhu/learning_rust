/*
//另一个没有所有权的数据类型是 slice。slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();
}

//字符串slice是String中一部分值的引用，它看起来像这样：
let s = String::from("hello, world");

let hello = &s[0..5];
let world = &s[6..11];
// ..=意味着包含最后的数字
let hello = &s[0..=4];
let world = &s[6..10];
//字符串字面值就是slice
//字符串slice作为参数
fn first_word(s: &String) -> &str {}
//而更有经验的 Rustacean 会编写出示例 4-9 中的签名，因为它使得可以对 String 值和 &str 值使用相同的函数：
fn first_word(s: &str) -> str {}

fn main() {
    let my_string = string::from("hello world");
    // first_word 中传入 `String` 的 slice
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";
    // first_word 中传入字符串字面值的 slice
    let word = first_word(&my_string_literal[..]);
    // 因为字符串字面值 **就是** 字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = first_word(my_string_literal);
}
*/

//其他类型的slice
fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[..];
    println!("the slice is {:?}", slice);
}