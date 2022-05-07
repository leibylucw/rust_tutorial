fn main() {
    let num = 12;

    let name = if num < 10 { "Tom" } else { "Luke" };
    println!("name is: {}", name);

    let a = [1, 2, 3, 4, 5];

    for element in a {
        println!("{}", element);
    }
}
