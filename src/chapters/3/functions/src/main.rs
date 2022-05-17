fn main() {
    let x = test(10);
    println!("The value of x is: {}", x);
}

fn test(x: i32) -> i32 {
    println!("This is the test function");

    x + 1
}
