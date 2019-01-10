fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    let s = "even more".to_string();

    // Both equivalent to:
    let s = String::from("initial contents");

    // This shit is UTF-8 encoded boooois:
    let s = String::from("🤷🏻‍♂️🔥");

    println!("{}", s);

    let mut s = String::from("foo");
    s.push_str("bar");

    println!("{}", s);

    let mut s = String::from("lo");
    // Push takes a single character:
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // must reference s2 because of the add-signature
    // which is called when we use +

    // The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String argument into a &str.

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    let s1 = String::from("hello");
    // let h = s1[0]; // Won't compile -- indexing for Strings in Rust doesn't work because of how Rust stores strings in memory

    // You should use ranges to create string slices with caution, because doing so can crash your program.

    // Instead, use chars-method
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // Or bytes-method
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // Getting grapheme clusters from strings is complex, so this functionality is not provided by the standard library. Crates are available on crates.io if this is the functionality you need.
}
