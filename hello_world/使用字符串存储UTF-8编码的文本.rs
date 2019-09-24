//新建字符串
//很多 Vec 可用的操作在 String 中同样可用，从以 new 函数创建字符串开始
let mut s = String::new();

let data = "initial contents";
let s = data.to_string();

//该方法也可直接用于字符串字面值
let s = "initial contents".to_string();
//也可以使用 String::from 函数来从字符串字面值创建 String
let s = String::from("initial contents");
//字符串是 UTF-8 编码的，所以可以包含任何可以正确编码的数据
let hello = String::from("你好")；
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שָׁלוֹם");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");
//更新字符串
//String 的大小可以增长其内容也可以改变，就像可以放入更多数据来改变 Vec 的内容一样。另外，可以方便的使用 + 运算符或 format! 宏来拼接 String 值。
//#使用 push_str 和 push 附加字符串
let mut s = String::from("foo");
s.push_str("bar");
//执行这两行代码之后 s 将会包含 foobar。push_str 方法获取字符串 slice，因为我们并不需要获取参数的所有权。
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s2);

//push 方法被定义为获取一个单独的字符作为参数，并附加到 String 中。
// 使用 push 方法将字母 l 加入 String 的代码。
let mut s = String::from("lo");
s.push('l');

//使用+运算符或format!宏拼接字符串
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;

//对于更为复杂的字符串链接，可以使用 format! 宏：
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);

//字节、标量值和字形簇！天呐！
//这引起了关于 UTF-8 的另外一个问题：从 Rust 的角度来讲，
//事实上有三种相关方式可以理解字符串：字节、标量值和字形簇（最接近人们眼中 字母 的概念）。

//索引字符串通常是一个坏点子，因为字符串索引应该返回的类型是不明确的：
//字节值、字符、字形簇或者字符串 slice。
let hello = "Здравствуйте";

let s = &hello[0..4];

//如果你需要操作单独的 Unicode 标量值，最好的选择是使用 chars 方法。对 “नमस्ते” 调用 chars 方法会将其分开并返回
// 六个 char 类型的值，接着就可以遍历其结果来访问每一个元素了：
for c in "नमस्ते".chars() {
    println!("{}", c);
}

//bytes 方法返回每一个原始字节
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
