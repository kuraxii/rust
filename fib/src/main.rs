use std::io;
fn fib_1(n: u32) -> u32
{
    if n == 1 || n == 2 
    {
        return 1;
    }else {
        return fib_1(n-1) + fib_1(n - 2);
    }

}

fn fib_2(n: u32) -> u32
{
    if n == 1 || n == 2 
    {
        return 1;
    }else {
        
        let mut b: u32 = 1;
        let mut c: u32 = 2;
        for i in 1..n-2
        {
            let a = b;
            b = c;
            c = a + b;
            
        }
        c
    }
    

}


fn main() {
    println!("{}", fib_1(8));
    
    print!("请输入：");
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("input err");
    let str: u32 = str.trim().parse().expect("not is a num");

    println!("{}", fib_2(str));
}
