
fn main() {
    // 新建一个空的vector
    let _v: Vec<i32> = Vec::new();
    
    // 通过初始值创建
    let mut _v = vec![1, 2, 3];

    // 插入值
    _v.push(4);
    _v.push(6);
    _v.push(7);
    _v.push(8);

    // 读取值
    // 有两种方法引用 vector 中储存的值：通过索引或使用 get 方法。

    // let third: &i32 = &_v[100];
    let third = match _v.get(100){
        None => None,
        m => m
    };

    // 当运行这段代码，你会发现对于第一个 [] 方法，当引用一个不存在的元素时 Rust 会造成panic。这个方法更适合当程序认为尝试访问超过 vector 结尾的元素是一个严重错误的情况，这时应该使程序崩溃。
    // 当 get 方法被传递了一个数组外的索引时，它不会 panic 而是返回 None 。当偶尔出现超过vector 范围的访问属于正常情况的时候可以考虑使用它。

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6);
    println!("The first element is: {first}");
    // 一旦程序获取了一个有效的引用，借用检查器将会执行所有权和借用规则（第四章讲到）来确保vector 内容的这个引用和任何其他引用保持有效。

    // 遍历
    let mut _v = vec![1, 3, 4, 5];
    for i in &mut _v
    {
        *i += 10;
        println!("{}", i);
    }
}
