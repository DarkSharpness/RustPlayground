mod slice_test;
mod struct_test;
mod reference_test;
mod array_test;

fn print_separator() {
    println!("--------------------------------------------------");
}

fn main() {
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