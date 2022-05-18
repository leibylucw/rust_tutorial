struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Named tuples as structs
// Both are different types even though they have the same types
struct Color (i32, i32, i32);
struct Point (i32, i32, i32);

// Unit-like struct
// which is just an empty struct
struct AlwaysEqual;

fn main() {
    let mut u1 = User {
        active: true,
        email: String::from("lucaswleiby@gmail.com"),
        username: String::from("leibylucw"),
        sign_in_count: 1,
    };

    u1.email = String::from("lwl10@pitt.edu");

    println!("u1's Username is {}", u1.username);
    println!("u1's email is {}", u1.email);

    let email2 = String::from("grenadine@bones.com");
    let username2 = String::from("grenadinethedog");
    let u2 = build_user(email2, username2);

    println!("u2's username is {}", u2.username);
    println!("u2's email is {}", u2.email);

    let u3 = User {
        email: u1.email,
        ..u1
    };

    println!("u3's username is {}", u3.username);
    println!("u3's email is {}", u3.email);
}

// Uses field init short-hand
fn build_user(email: String, username: String) -> User {
    let u = User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    };

    u
}
