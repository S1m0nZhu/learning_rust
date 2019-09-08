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
    mod instrument {
        fn clarinet() {
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