fn area(width: u32, height: u32) -> u32
{
    width * height
}

// 改进  元组
fn _area(rectangles: (u32, u32)) -> u32
{
    rectangles.0 * rectangles.1
}

// 改进  结构体
#[derive(Debug)]
struct  Rectangles
{
    width: u32,
    height: u32
}

impl Rectangles {  // 定义结构体方法  们第一个参数总是 self
    fn area(&self) -> u32
    {
        self.height * self.width
    }
    
}

fn __area(rectangles: &Rectangles) -> u32
{
    rectangles.height * rectangles.width
}

fn rectangles_build(width: u32, height: u32) -> Rectangles
{
    Rectangles { 
        width, 
        height 
    }
}



fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
        );
    
    let rectangles1 = (width1, height1);
    println!(
        "The area of the rectangle is {} square pixels.",
        _area(rectangles1)
        );

    let rectangles2 = rectangles_build(width1, height1);
    println!(
        "The area of the rectangle is {} square pixels.",
        __area(&rectangles2)
        );
    println!("The area of the rectangle is {} square pixels.",
    rectangles2.area());

    println!("rect2 is: {:#?}", rectangles2);
}
