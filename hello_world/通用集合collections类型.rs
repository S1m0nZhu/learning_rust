//为了创建一个新的空 vector，可以调用 Vec::new 函数
let v: Vec<i32> = vec::new();
//新建一个拥有值1、2、3的Vec<i32>
let v = vec![1, 2, 3];
//新建一个vector并向其增加元素，可以使用push方法
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);

//丢弃vector时也会丢弃其所有元素
{
    let v =Vec![1, 2, 3, 4];
    //处理变量v
}//<- 这里v离开作用域并被丢弃

//读读取vector的元素
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
//遍历vector中的元素
let v = vec![100, 32, 57];
for i in &v {
    *i += 50;
    println!("{}", i);
}
//使用枚举来存储多种类型
enum SpreadsheerCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadSheetCell::Int(3),
    SpreadSheetCell::Tect(String::from("blue")),
    SpreadSheetCell::Float(10.12),
]