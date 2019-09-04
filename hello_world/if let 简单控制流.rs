//if let 语法让我们以一种不那么冗长的方式结合 if 和 let，来处理只匹配一个模式的值而忽略其他模式的情况。
let some_u8_value = some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
//以下if let与上面match行为一致
if let Some(3) = some_u8_value {
    println!("three");
}
//示例6-4中Coin枚举的定义，Quarter成员也包含一个UsState值。如果想要计数所有不是25美分的硬币的同事也报告25美分硬币所属的州，可以使用这样一个match表达式
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}", state),
    _ => count += 1,
}
//或者可以使用这样的if let 和 else表达式
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}