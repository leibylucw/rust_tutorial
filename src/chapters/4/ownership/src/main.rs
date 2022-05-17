fn main() {
    let s1 = String::from("Hello");

    // s1 has moved inot s2
    let s2 = s1;

    println!("s2 is {}", s2);

    // s3 is a clone of s2, so both are valid
    let s3 = s2.clone();

    println!("s3 is {}", s3);

    let s4 = gives_ownership();

    println!("s4 is {}", s4);

    let s5 = String::from("Goodbye");

    let s5 = takes_and_gives_back(s5);

    println!("s5 is {}", s5);

    let x = 5;

    // y just copies what's in x
    // both are fixed-sized types encoded into the binary file
    // Cloning does nothing for us here
    let y = x;

    println!("x is {}", x);
    println!("y is {}", y);
}

// Returns a String
fn gives_ownership() -> String {
    let s = String::from("I came from gives_ownership()");

    s
}

// Returns same String that was passed in
fn takes_and_gives_back(s: String) -> String {
    s
}
