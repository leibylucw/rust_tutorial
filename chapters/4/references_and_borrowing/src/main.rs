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

    let s6 = String::from("Luke Leiby");
    let s7 = first_word(&s6);

    println!("s6 is {}", s6);
    println!("s7 is {}", s7);

    let a = [1, 2, 3, 4, 5];
    let a_ref = &a[1..3];

    println!("a's length is {}", a.len());
    println!("a_ref's length is {}", a_ref.len());
}

fn get_len(s: &String) -> usize {
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
