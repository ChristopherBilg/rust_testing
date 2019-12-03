fn main() {
    // Arrays in Rust must be initialized at compile time
    let x: [i32; 5] = [0, 1, 2, 3, 4];

    print_array(x);
    println!("");
}

fn print_array(arr: [i32; 5]) {
    for item in arr.iter() {
        print!("{} ", item);
    }
}
