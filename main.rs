// https://www.codingame.com/playgrounds/365/getting-started-with-rust/primitive-data-types

fn main() {
    // Arrays in Rust must be initialized at compile time
    // Array size = 20, and initialized to 5
    const SIZE: usize = 20;
    const INIT_VALUE: i32 = 5;
    let x: [i32; SIZE] = [INIT_VALUE; SIZE];

    print_array(&x);
    println!("");
}

fn print_array(arr: &[i32]) {
    for item in arr.iter() {
        print!("{} ", item);
    }
}
