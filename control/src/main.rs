fn ifelse_statement() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let str = if number < 9 { "less" } else { "more or equal" };
    println!("{}", str);
}

fn loop_statement() {
    let mut number = 0;
    // loop 语句
    loop {
        println!("again! {}", number);
        number += 1;
        if number % 2 == 0 {
            continue;
        }

        if number > 100 {
            break;
        }
    }

    // loop表达式做返回值
    number = 0;
    let x = loop {
        println!("again! {}", number);
        number += 1;
        if number % 2 == 1 {
            continue;
        }

        if number > 100 {
            break number;
        }
    };
    println!("x = {}", x);

    let mut count = 0;
    // 循环标签
    'loop1: loop {
        println!("count = {count}");
        let mut remaining = 10;    
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'loop1;
            }
            remaining -= 1;
        }
        count += 1;
    }
}

fn for_statement()
{
    for i in 1..10
    {
        println!("{}", i);
    }
}
fn main() {
    ifelse_statement();
    loop_statement();
    for_statement();
}
