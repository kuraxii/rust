// 定义
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


// 结构体初始化
fn bulid_user(username: String, email: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 66,
    }
}

// 改进，字段初始化简写语法. 参数名和字段名相同
fn _bulid_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 66,
    }
}




fn main() {
    let user1 = User {
        active: true,
        username: String::from("zzj"),
        email: String::from("zj.zhu.cn@gamil.com"),
        sign_in_count: 123456,
    };

    println!(
        "{}\n{}\n{}\n{}\n",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );

    let user2 = bulid_user(String::from("zzz"), String::from("zj@gamil.com")); 
    println!(
        "{}\n{}\n{}\n{}\n",
        user2.active, user2.username, user2.email, user2.sign_in_count
    );

    let user2 = User{  // 结构体更新 结构体更新中的复制类似于`=`, 因为它移动了数据，被移动的数据无法使用
        email: String::from("22@gmail.com"),
        ..user1
    };
    println!(
        "{}\n{}\n{}\n{}\n",
        user2.active, user2.username, user2.email, user2.sign_in_count
    );

    
    
    
}
