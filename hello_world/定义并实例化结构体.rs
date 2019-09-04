struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    let mut user1 = User {
        email: String::from("zm349@qq.com"),
        username: String::from("S1m0nZhu"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("zhumeng369@gmail.com");
}

//build_user 函数获取 email 和用户名并返回 User 实例
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}