// newtype pattern, wrapping i32
struct Meters(i32);

// type alias. just an alias, no static
// type checking like with newtype pattern
type Kilometers = i32;

// type alias is useful where a type is verbose, like
type Thunk = Box<dyn Fn() + Send + 'static>;

// instead of f: Box<dyn Fn() + Send + 'static>
fn takes_long_type(f: Thunk) {
    // --snip--
    println!("Wow thunk");
}

fn main() {
    println!("Hello, world!");

    let f: Thunk = Box::new(|| println!("hi"));
}
