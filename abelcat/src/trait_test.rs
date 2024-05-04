pub trait DemoTrait {
    fn demo(&self) -> String;
}

pub struct MyStr { pub data: String, }
pub struct MyInt { pub data: usize,  }

impl DemoTrait for MyStr {
    fn demo(&self) -> String {
        self.data.clone()
    }
}

impl DemoTrait for MyInt {
    fn demo(&self) -> String {
        self.data.to_string()
    }
}

pub fn main() {
    type MyBox = Box<dyn DemoTrait>;

    let str = Box::new(MyStr { data: "Hello".to_string() }) as MyBox;
    let int = Box::new(MyInt { data: 123 }) as MyBox;

    println!("{}", str.demo());
    println!("{}", int.demo());
}
