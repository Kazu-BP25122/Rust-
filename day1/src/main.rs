fn main() {
    println!("Hello, world!");
    let x:i32 = 3;
    let y:i32 = 7;
    println!("x + y = {}", add(x, y));

    let a: [i32; 3] = [4, 5, 6];
    println!("a[1] = {}", a[1]);
}

fn add(x:i32, y:i32) -> i32 {
    x + y
}