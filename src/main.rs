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