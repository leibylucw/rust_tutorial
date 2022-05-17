fn main() {
    let mut s1 = String::from("Hello");

    // A reference to s1
    // s1 is still valid
    let s1_len = get_len(&s1);

    println!("The length of s1 is {}", s1_len);

    // You can have multiple immutable references
    // since none of them can change the value out from under the others
    let s2 = &s1;
    let s3 = &s1;

    println!("s2 is {}", s2);
    println!("s3 is {}", s3);

    // Only one mutable reference at a time
    let s4 = &mut s1;

    println!("s4 is {}", s4);

    // This is fine since references drop scope the last time they're used
    let s5 = &mut s1;

    println!("s5 is {}", s5);
}

fn get_len(s: &String) -> usize {
    s.len()
}
