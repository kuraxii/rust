#[allow(dead_code)]  // 允许函数不被调用
fn add(x: i32, y: i32) -> i32
{
    x + y
}

fn expression() -> i32
{
    let y = 6;
    let x =  y ;
    println!("x = {}, y = {}", x, y);
    y
}

fn main() {
    let a = 16;
    let b = 20;
    println!("{} + {} = {}", a, b, add(a, b));
}
