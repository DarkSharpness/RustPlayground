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

    let str = test_str();
    let int = test_int();

    unsafe { STR.data = "World".to_string() };
    unsafe { INT.data = 114514 };

    println!("{}", unsafe { (*str).demo() });
    println!("{}", unsafe { (*int).demo() });
}

static mut STR : MyStr = MyStr { data: String::new() };
static mut INT : MyInt = MyInt { data: 123 };

#[allow(static_mut_refs)]
fn test_str() -> *mut dyn DemoTrait {
    return unsafe { &mut STR };
}

#[allow(static_mut_refs)]
fn test_int() -> *mut dyn DemoTrait {
    return unsafe { &mut INT };
}
