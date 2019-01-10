fn main() {
    let a = "hello"; // string literal

    println!("{}", a);

    let mut s = String::from("hello"); // String type

    s.push_str(", world!");

    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1;

    //println!("{}, world!", s1); // won't compile, s1 was "moved" and therefore invalid. Rust never automatically copies heap data, like Strings

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y); // will compile, since Rust automatically will copy stack data, like integers. It has the Copy trait

    // Here are some of the types that are Copy:
    // * All the integer types, such as u32.
    // * The Boolean type, bool, with values true and false.
    // * All the floating point types, such as f64.
    // * The character type, char.
    // * Tuples, if they only contain types that are also Copy. For example, (i32, i32) is Copy, but (i32, String) is not.

    // Passing a variable to a function will move or copy, just as assignment does:
    takes_ownership(s2);

    // println!("{}", s2); won't compile since its been moved

    // integer type, so is copied
    makes_copy(x);

    // integer types are copied, so not invalid:
    println!("{}", x);

    let s3 = String::from("hey");

    let s3 = takes_and_gives_back(s3);

    println!("{} there", s3);

    let s4 = gives_ownership();

    println!("{} this is mine now", s4);

    // Let calculcate_length borrow s4, pass as a reference:
    let s4len = calculate_length(&s4);

    println!("The length of {} is {}", s4, s4len);

    let mut s5 = String::from("Nope");

    change(&mut s5);

    println!("Hey mutated? {}", s5);

    let r1 = &mut s5;
    //let r2 = &mut s5; Cannot borrow twice, will not compile

    // brackets denote new scope:
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    println!("{} {}", r1, r2);

    // In short, about references:
    // * At any given time, you can have either one mutable reference or any number of immutable references.
    // * References must always be valid.

    // A string slice is a reference to part of a String, and it looks like this:

    let s6 = String::from("hello world");

    let hello = &s6[0..5];
    let world = &s6[6..11];

    println!("{} {}", hello, world);

    let mut s7 = String::from("hello world");

    let word = first_word(&s7);

    //s7.clear(); // Won't compile because we have an immutable reference to s7 (word), and therefore can't take a mutable with clear()

    println!("the first word is: {}", word);

    // String literals are of type &str, a slice pointing to a speicifc point of the binary. An immutable reference
    let str_literal = "Hello, world!";

    let arr = [1, 2, 3, 4, 5];

    let arr_slice = &arr[1..3];
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("{} man", a_string);
    a_string
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    // some_string is moved to calling scope out of function
    some_string
}

// Does not take ownership of s
fn calculate_length(s: &String) -> usize {
    // s.push_str(", world"); wont work since it has been borrowed, references are immutable by default
    s.len()
}

// Mutable reference:
fn change(some_string: &mut String) {
    some_string.push_str("... JK of course!");
}

// Return a String-slice pointing to the first word
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
