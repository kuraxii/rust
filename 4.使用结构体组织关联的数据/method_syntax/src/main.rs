struct Rectangles {
    width: u32,
    height: u32,
}

impl Rectangles {  // 每个结构体都允许拥有多个 impl 块, 最终会合并为一个
    // 定义结构体方法  它们第一个参数总是 self
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangles) -> bool // 更多参数的方法
    {
        self.height > other.height && self.width > other.width  
    }

    // 关联
    fn build(width: u32, height: u32) -> Self {  // 关键字 Self 在函数的返回类型中代指在 impl 关键字后出现的类型，在这里是 Rectangle
        Rectangles { width, height }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let rectangles2 = Rectangles::build(width1, height1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangles2.area()
    );

    let rect1 = Rectangles::build(30, 50);
    let rect2 = Rectangles::build(10, 40);
    let rect3 = Rectangles::build(60, 45);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}


// 结构体让你可以创建出在你的领域中有意义的自定义类型。通过结构体，我们可以将相关联的数据片段联系起来并命名它们，这样可以使得代码更加清晰。在 impl 块中，你可以定义与你的类型相关联的函数，而方法是一种相关联的函数，让你指定结构体的实例所具有的行为。
//但结构体并不是创建自定义类型的唯一方法：让我们转向 Rust 的枚举功能，为你的工具箱再添一个工具。
