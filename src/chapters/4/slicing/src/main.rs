fn main() {
    let s6 = String::from("Luke Leiby");
    let s7 = first_word(&s6);

    println!("s6 is {}", s6);
    println!("s7 is {}", s7);

    let a = [1, 2, 3, 4, 5];
    let a_ref = &a[1..3];

    println!("a's length is {}", a.len());
    println!("a_ref's length is {}", a_ref.len());    
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
