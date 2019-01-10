use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec!["Blue", "Yellow"];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let score = scores.get(&"Blue");
    match score {
        None => println!("No score!"),
        Some(num) => println!("{}", num),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    // insert overwrites old value:
    scores.insert(String::from("Blue"), 25);

    // .entry checks if exists, if not, .or_insert will insert 50
    scores.entry(String::from("Blue")).or_insert(50);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // * dereferences count and updates it
        *count += 1;
    }

    println!("{:?}", map);
}
