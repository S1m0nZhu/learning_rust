//_通配符
fn match_control_flow() {
    let some_u8_value = 18;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
fn main() {
    match_control_flow();
}