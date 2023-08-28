fn takes_ownership(some_string: String)
{ //some_string 进入作用域
    println!("{}", some_string);
} // some_string 移出作用域并调用drop方法
  // 堆空间被释放


fn make_copy(some_integer: u32)
{ // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处

fn gives_ownership() -> String
{
    let some_string = String::from("hello"); // some_string 进入作用域。
    some_string     // 返回 some_string
                    // 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String
{
    a_string // 返回 a_string 并移出给调用的函数
}

fn calculate_length(s: String) -> (String, usize)
{
    let length = s.len();

    (s, length)
}



fn main() {
    let mut x = 5;
    let y = x;
    x += 1;
    println!("{},{}", x, y);

    // 移动语义 被移动的变量是无效的
    let s_1 = String::from("hello");
    let mut str_1 = s_1;
    str_1.push_str("world");
    println!("str_1={}", str_1);

    // 克隆 深拷贝
    let s_2 = String::from("hello");
    let mut str_2 = s_2.clone();
    str_2.push_str("world");
    println!("s={}, str_2={}", s_2, str_2);

    let s = String::from("hello");

    // 将值传递给函数与给变量赋值的原理相似。向函数传递值可能会移动或者复制，就像赋值语句一样。
    takes_ownership(s); // s移动到函数里
    // println!("{}", s);  s在这里无效

    let x = 5;
    make_copy(x);   // x 应该移动函数里，
                    // 但 i32 是 Copy 的，
                    // 所以在后面可继续使用 x

    // 返回值也可以转移所有权
    let s_3 = gives_ownership();
    println!("{}", s_3);

    let s_4 = takes_and_gives_back(s_3);
    // println!("{}", s_3);  // s_3已经被转移，所以是无效的
    println!("{}", s_4);

    let (s_5, len) = calculate_length(s_4);
    println!("s_5={}, len={}", s_5, len);
    
   

}
