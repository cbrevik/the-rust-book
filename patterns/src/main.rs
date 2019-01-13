fn main() {
    // let PATTERN = STATEMENT
    // E.g. tuple destructering
    let (x, y, z) = (1, 2, 3);
    println!("x: {}, y: {}, z: {}", x, y, z);

    let point = (3, 5);
    print_coordinates(&point);

    // Can also pattern match in closure params:
    let close = |&(x, _): &(i32, _)| {
        println!("Just interested in x: {}", x);
    };

    close(&point);

    let point = (3, "hello");

    // Can't do this because the above usage inferred the second tuple value as i32:
    // close(&point);
    // If I instead comment out the first usage of close, then it would work

    // Patterns come in two forms: refutable and irrefutable. Patterns that will match for any possible value passed are irrefutable.
    // Patterns that can fail to match for some possible value are refutable.
    // Function parameters, let statements, and for loops can only accept irrefutable patterns, because the program cannot do anything meaningful when values don’t match.
    // The if let and while let expressions only accept refutable patterns, because by definition they’re intended to handle possible failure

    // Can have multiple patterns for one arm:
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Also ranges:
    let x = 5;

    match x {
        1...5 => println!("one through five"),
        _ => println!("something else"),
    }

    // Ranges are only allowed with numeric values or char values, because the compiler checks that the range isn’t empty at compile time.
    let x = 'c';

    match x {
        'a'...'j' => println!("early ASCII letter"),
        'k'...'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Can use patterns to destruct structs, enums, tuples and references
    let p = Point { x: 0, y: 7 };

    // Can rename:
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    println!("{}, {}", x, y);

    // Match literal values, y is 0, or second arm, x is 0:
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    };

    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    // When the value we’re matching to our pattern contains a reference, we need to destructure the reference from the value, which we can do by specifying a & in the pattern. Doing so lets us get a variable holding the value that the reference points to rather than getting a variable that holds the reference.
    let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
    println!("Sum of squares: {}", sum_of_squares);

    // This technique is especially useful in closures where we have iterators that iterate over references, but we want to use the values in the closure rather than the references.

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet: {}, inches: {}, x: {}, y: {}", feet, inches, x, y);

    // Start a variable name with _ to suppress warning about unsused variable:
    let _unused = 5;

    let s = Some(String::from("Hello!"));

    // Must use _ here and not _s, because _s would take ownership of the value, _ does not, gotcha:
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    let origin = Point { x: 0, y: 0 };

    // .. ignores the remaining values 🔥
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    let num = Some(4);

    // A match guard is an additional if condition, which are useful for expressing more complex ideas than a pattern alone allows:
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    // The at operator (@) lets us create a variable that holds a value at the same time we’re testing that value to see whether it matches a pattern.
    match msg {
        Message::Hello {
            id: id_variable @ 3...7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10...12 } => println!("Found an id in another range"),
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

// Function parameter destructering/pattern:
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

struct Point {
    x: i32,
    y: i32,
}
