enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(u8)
}
enum squ{
    Pp,
    Qq
}

// 匹配枚举绑定值
fn value_in_cents(coin: Coin) ->u8{
    match coin {
        Coin::Dime => 10,
        Coin::Nickel => 5,
        Coin::Penny => 1,
        Coin::Quarter(i) => i,
        
    }
}

// 匹配Option<T>
fn plus_one(x: Option<i32>) ->Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1)
    }

}
// 将 match 与枚举相结合在很多场景中都是有用的。你会在 Rust 代码中看到很多这样的模式：
// match 一个枚举，绑定其中的值到一个变量，接着根据其值执行代码。这在一开始有点复杂，不过一旦习惯了，你会希望所有语言都拥有它！这一直是用户的最爱。





fn main() {

 
    

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    
    
// match 的语法糖  let if
//  正常的match
//  匹配Some 
    let config_max = Some(3u8);
    match config_max{
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => ()
    }

// let if模式下的匹配  = 后面的（模式）匹配前面的（表达式）
    if let Some(i) = config_max{
        println!("The maximum is configured to be {}", i);
    }
    // 如果值是 Some ，我们希望打印出 Some 成员中的值，这个值被绑定到模式中的 max 变量里。对于 None 值我们不希望做任何操作。为了满足 match 表达式（穷尽性）的要求，必须在处理完这唯一的成员后加上 _ => () ，这样也要增加很多烦人的样板代码。


}

// 现在我们涉及到了如何使用枚举来创建有一系列可列举值的自定义类型。我们也展示了标准库的Option<T> 类型是如何帮助你利用类型系统来避免出错的。当枚举值包含数据时，你可以根据需要处理多少情况来选择使用 match 或 if let 来获取并使用这些值。你的 Rust 程序现在能够使用结构体和枚举在自己的作用域内表现其内容了。在你的 API 中使用自定义类型保证了类型安全：编译器会确保你的函数只会得到它期望的类型的值。为了向你的用户提供一个组织良好的 API，它使用起来很直观并且只向用户暴露他们确实需要的部分，那么现在就让我们转向 Rust 的模块系统吧。