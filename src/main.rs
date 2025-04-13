extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    let a: i32 = 3;
    let b: i32 = 2;

    let result: i32 = unsafe { add(a, b) };

    println!("3 + 2 = {}", result);
}