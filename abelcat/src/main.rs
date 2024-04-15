mod slice_test;
mod struct_test;
mod reference_test;
mod array_test;

fn print_separator() {
    println!("--------------------------------------------------");
}


struct Tester { pub x : i32, }

fn drop_test() {
    let _x = Tester { x: 10 };
    let _y = _x;
}

fn main() {
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
