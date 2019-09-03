//fn main() {
//    let s1 = String::from("hello");
//
//    let len = calculate_length(&s1);
//
//    println!("The length of '{}' is {},", s1, len);
//}
//
//fn calculate_length(s:&String) -> usize { //&引用， *解引用，获取引用作为函数参数称为借用
//    s.len()
//}

fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
    println!("{}", some_string)
}