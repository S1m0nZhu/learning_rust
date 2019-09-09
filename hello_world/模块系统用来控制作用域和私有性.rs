//模块，一个组织代码和控制路径私有性的方式
//路径，一个命名项（item）的方式
//use 关键字用来将路径引入作用域
//pub 关键字使项变为公有
//as 关键字用于将项引入作用域时进行重命名
//使用外部包
//嵌套路径用来消除大量的 use 语句
//使用 glob 运算符将模块的所有内容引入作用域
//如何将不同模块分割到单独的文件中

//绝对路径（absolute path）从 crate 根开始，以 crate 名或者字面值 crate 开头。
//相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头。
//绝对路径和相对路径都后跟一个或多个由双冒号（::）分割的标识符。
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            //函数体
        }
    }
}

fn main() {
    //绝对路径
    crate::sound::instrument::clarinet();
    //相对路径
    sound::instrument::clarinet();
}
//使用super开始相对路径
mod instrument {
    fn clarinet() {
        super::breathe_in();
    }
}
fn breathe_in() {
    //函数体
}

//对结构体和枚举使用pub
mod plant {
    pub struct Vegetable {
        pub name:String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

fn main() {
    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);
}
//将枚举设计为公有会使其所有成员公有
mod menu {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn main() {
    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;
}
//使用use关键字将名称引入作用域
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            //函数体
        }
    }
}

use crate::sound::instrument;
//use self::sound::instrument;

fn main() {
    instrument::clarinet();
    instrument::clarinet();
    instrument::clarinet();
}
//use函数路径使用习惯vs其他项
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            //函数体
        }
    }
}
use crate::sound::instrument::clarinet;
fn main() {
    clarinet();
    clarinet();
    clarinet();
}
//将HashMap引入作用域的习惯用法
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
//将HashMap引入作用域的非习惯用法
use std::collections;

fn main() {
    let mut map = collections:HashMap::new();
    map.insert(1, 2);
}
//将两个同名类型引入作用域必须使用父模块
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
#    OK(())
}
fn function2() -> io::Result<()> {
    #Ok(())
}
//通过as关键字重命名引入作用域的类型
use std::fmt::Result;
use std::io::REsult as IoResult;

fn function1() -> Result {
    #    OK(())
}
fn function2() -> ioResult<()> {
    #Ok(())
}
//通过pub use 重导出名称
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            //函数体
        }
    }
}

mod performance_group {
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();

    }
}
fn main() {
    performance_group::clarinet_trio();
    performance_group::instrument::clarinet();
}
//使用外部包 在Cargo.toml中加入行
[dependencies]
rand = "0.5.5"