fn calculate_length(s: &String) -> usize
{
    // s.push('c'); rust 引用是不允许使用的
    s.len()
}// 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
// 所以什么也不会发生

fn change(some_string: &mut String)
{
    some_string.push_str(",world");
}

// // 悬垂引用
// fn dangling_references() -> &String 
// {
//     let s = String::from("hello");
//     &s // 返回字符串 s 的引用
// }   // 这里 s 离开作用域并被丢弃。其内存被释放。
//     // 危险！

// 修正后

fn dangling_references() -> String 
{
    let s = String::from("hello");
    s // 返回字符串 s 的引用
}   // 将字符串所有权转让给调用者

fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("s1={}, len={}", s1, len);
    // 注意变量声明和函数返回值中的所有元组代码都消失了。其次，注意我们传递 &s1 给calculate_length ，同时在函数定义中，我们获取 &String 而不是 String 。这些 & 符号就是引用，它们允许你使用值但不获取其所有权。

    change(&mut s1);
    // 可变引用有一个很大的限制：如果你有一个对该变量的可变引用，你就不能再创建对该变量的引用。这些尝试创建两个 s 的可变引用的代码会失败
    println!("change:s1={}", s1);

    let s2: &mut String = &mut s1;
    s2.push_str("111");
    s1.push_str("222");

    let s3 = dangling_references();
    println!("s3={}", s3);




}


