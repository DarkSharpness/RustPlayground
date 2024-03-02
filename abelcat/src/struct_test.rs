#[derive(Debug, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn test(&self) -> (u32,u32) {
        (self.width, self.height)
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn output(x : Rectangle) {
    println!("The area of the rectangle is {} square pixels. {} x {}",
        x.area(), x.test().0, x.test().1);
}

pub fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle::square(30);

    output(rect1.clone());
    output(rect2);
    output(rect1);

    dbg!("end of this test");
}
