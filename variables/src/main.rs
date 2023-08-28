
// 变量
#[allow(dead_code)]  // 允许函数不被调用
fn value() 
{
    
    let mut x = 5;  // rust变量默认不可修改 可修改变量使用mut修饰
    println!("x= {x}");

    x = 6;
    println!("x = {x}");
}

// 常量
#[allow(dead_code)]  // 允许函数不被调用
fn const_value() 
{
    
    const CONST_VALUE: u32 = 4;  // rust常量命名使用全大写以及下划线, 并且应指定数据类型
    println!("CONST_VALUE= {}", CONST_VALUE);
}

// 隐藏 Shadowing
#[allow(dead_code)]  // 允许函数不被调用
fn shadow()
{
    
    let shadow_x = 5;

    let shadow_x = shadow_x + 1;

    {
        let shadow_x = shadow_x * 2;
        println!("The value of shadow_x in the inner scope is: {shadow_x}");
    }

    println!("The value of shadow_x is: {shadow_x}");
    /*  隐藏与将变量标记为 mut 是有区别的。当不小心尝试对变量重新赋值时，如果没有使用 let 关键字，就会导致编译时错误。通过使用 let，我们可以用这个值进行一些计算，不过计算完之后变量仍然是不可变的。
    mut 与隐藏的另一个区别是，当再次使用 let 时，实际上创建了一个新变量，我们可以改变值的类型，并且复用这个名字。
    */
}

// 数据溢出
#[allow(dead_code)]  // 允许函数不被调用
fn data_overflow() 
{
    
    let mut _z: u8 = 255;
    //_z += 1;
    println!("z = {_z}");
    /* 
    当在 debug 模式编译时，Rust 检查这类问题并使程序 panic，这个术语被 Rust 用来表明程序因错误而退出
    使用 --release flag 在 release 模式中构建时，Rust 不会检测会导致 panic 的整型溢出。相反发生整型溢出时，Rust 会进行一种被称为二进制补码 wrapping（two’s complement wrapping）的操作。简而言之，比此类型能容纳最大值还大的值会回绕到最小值，值 256 变成 0，值 257 变成 1，依此类推。程序不会 panic，不过变量可能也不会是你所期望的值。依赖整型溢出 wrapping 的行为被认为是一种错误。
    */

}

// 字符类型
#[allow(dead_code)]  // 允许函数不被调用
fn char_type()  
{
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    /* 
    rust字符类型占4个字节，并代表了一个 Unicode 标量值（Unicode Scalar Value），这意味着它可以比 ASCII 表示更多内容。在 Rust 中，带变音符号的字母（Accented letters），中文、日文、韩文等字符，emoji（绘文字）以及零长度的空白字符都是有效的 char 值。Unicode 标量值包含从 U+0000 到 U+D7FF 和 U+E000 到 U+10FFFF 在内的值。
    */

    println!("c= {c}, z = {z}, heart_eye_cat = {}", heart_eyed_cat);
}

// 复合类型
#[allow(dead_code)]  // 允许函数不被调用
fn compound_type()
{
    // tuple 元组
    let tup: (i32, f64, &str)= (612, 5.01, "45");
    println!("tup({},{},{})", tup.0, tup.1, tup.2);
    
    // array 数组
    let array = [1,2,3,4,5,6];
    println!("{}", array[0]);
    for i in array
    {
        println!("{}", i);
    }
}

fn main() {
    
    // value();
    // const_value();
    // shadow();
    // data_overflow();
    // char_type();
    compound_type();

    
    
}
