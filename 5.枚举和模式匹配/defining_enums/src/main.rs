
enum IpaddrKind{
    V4, 
    V6
}

enum IpAddr {  // 将数据类型绑定到枚举成员上
    V4(String),
    V6(String)    
}

struct _Ipaddr{
    kind: IpaddrKind,
    addr: String
}

impl _Ipaddr{
    fn build(kind: IpaddrKind, addr: &str) ->Self
    {
        _Ipaddr { kind, addr: String::from(addr) }
    }


}


enum _Message {  // 一个复杂的枚举
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// 这个枚举有四个含有不同类型的成员：
// Quit 没有关联任何数据。
// Move 类似结构体包含命名字段。
// Write 包含单独一个 String 。
// ChangeColor 包含三个 i32 。

fn main() {
    // 使用结构体存储ip
    let _home = _Ipaddr::build(IpaddrKind::V4, "127.0.1.1");
    let _loopback = _Ipaddr::build(IpaddrKind::V6, "::1"); 

    // 改进 使用枚举绑定数据类型
    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));




}
