//! Not an actual gui lib, just something to demonstrate OOP

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // This works differently than defining a struct that uses a generic type parameter with trait bounds. A generic type parameter can only be substituted with one concrete type at a time, whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime.
    // E.g. pub components: Vec<T>,
    // This restricts us to a Screen instance that has a list of components all of type Button or all of type TextField.
    pub components: Vec<Box<dyn Draw>>,
}

// When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t know all the types that might be used with the code that is using trait objects, so it doesn’t know which method implemented on which type to call. Instead, at runtime, Rust uses the pointers inside the trait object to know which method to call. I.e. Box<dyn Draw>
// It does not perform monomorphization, where the compiler generates nongeneric implementations of functions and methods for each concrete type that we use in place of a generic type parameter, which it can call with static dispatch

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
