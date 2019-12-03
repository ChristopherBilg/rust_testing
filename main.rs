// https://www.codingame.com/playgrounds/365/getting-started-with-rust/primitive-data-types

fn main() {
    // Arrays in Rust must be initialized at compile time
    // Array size = 10, and initialized to 5
    const SIZE: usize = 10;
    let initial_value: i32 = 5;
    let x: [i32; SIZE] = [initial_value; SIZE];

    print_array(&x);
    println!("");
}

fn print_array(arr: &[i32]) {
    for item in arr.iter() {
        print!("{} ", item);
    }
}
