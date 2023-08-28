fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    // iter 方法返回集合中的每一个元素，而enumerate 包装了 iter 的结果，将这些元素作为元组的一部分来返回。 enumerate 返回的元组中，第一个元素是索引，第二个元素是集合中元素的引用。
    s.len()
}

//改进
fn _first_word(s: &String) -> &str {  // 返回一个slice引用

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    
    &s[..]
}

// 改进
fn __first_word(s: &str) -> &str
{

    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return &s[0..i];
        }
    }
    s
}

// 字符串面值就是slice，类型为&str 是一个不可变引用
fn main() {
    let mut str = String::from("123 456");
    let first_word = __first_word(&str[..]);


    println!("first word:{}", first_word);
    str.clear();
}
