mod slice_test;
mod struct_test;
mod reference_test;
mod array_test;
mod trait_test;

fn print_separator() {
    println!("--------------------------------------------------");
}


struct Tester { pub x : i32, }
#[derive(Clone)] // No copy for a type with drop
struct Copyer { pub x : i32, }

fn drop_test() {
    let _x = Tester { x: 10 };
    let _y = _x;
    let _z = Copyer { x: 20 };
    let _w = _z.clone();
}

fn main() {
    trait_test::main();

    print_separator();

    drop_test();

    print_separator();

    slice_test::main();

    print_separator();

    struct_test::main();

    print_separator();

    reference_test::main();

    print_separator();

    array_test::main();

    print_separator();
}


impl Drop for Tester {
    fn drop(&mut self) {
        println!("Dropping Tester with x = {}", self.x);
    }
}

impl Drop for Copyer {
    fn drop(&mut self) {
        println!("Dropping Copyer with x = {}", self.x);
    }
}