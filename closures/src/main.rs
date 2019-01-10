use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

struct Cacher<T, P, R>
where
    T: Fn(P) -> R,
    P: Hash + Eq + Copy,
    R: Copy,
{
    calculation: T,
    values: HashMap<P, Option<R>>,
}

impl<T, P, R> Cacher<T, P, R>
where
    T: Fn(P) -> R,
    P: Hash + Eq + Copy,
    R: Copy,
{
    fn new(calculation: T) -> Cacher<T, P, R> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: P) -> R {
        match self.values.get(&arg) {
            Some(kv) => match kv {
                Some(v) => v.clone(),
                None => {
                    let v = (self.calculation)(arg);
                    self.values.insert(arg, Some(v));
                    v
                }
            },
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, Some(v));
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

    #[test]
    fn string_slice_with_usize() {
        let mut c = Cacher::new(|a: &str| a.len());

        let length = c.value("yep");

        assert_eq!(length, 3);
    }

    #[test]
    fn capture_the_environment() {
        let x = 4;

        let equal_to_x = |z| z == x;

        let y = 4;

        assert!(equal_to_x(y));
    }

    #[test]
    fn lets_move_stuff() {
        let x = vec![1, 2, 3];

        let equal_to_x = move |z| z == x;

        let y = vec![1, 2, 3];

        assert!(equal_to_x(y));
    }
}
