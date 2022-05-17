fn main() {
    // The mut keyword makes a variable mutable
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    const MY_CONST: u32 = 1;
    println!("The value of MY_CONST is: {}", MY_CONST);

    // Shadowing
    let y = 2;

    // Use this:
    let y = y + 1;
    // instead of this:
    // y = y + 1;

    // Shadowing makes use of the let keyword twice
    // if using mut while switching types, shadowing cannot work
    // because variables cannot change types
    // mut is different than shadowing

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y in the outer scope is: {}", y);
}
