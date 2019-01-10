use std::io;

fn main() {
    println!("Hello Fibonacci!");

    let nth: u32;

    loop {
        println!("Please input the nth Fibonacci number you would like to go up to.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        nth = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break;
    }

    let mut current_sum: u32 = 1;

    if nth == 0 {
        current_sum = 0;
    } else if nth > 2 {
        let mut previous_sum: u32 = 1;

        let mut iteration = nth - 2;

        while iteration > 0 {
            let new_sum = previous_sum + current_sum;
            previous_sum = current_sum;
            current_sum = new_sum;
            iteration = iteration - 1;
        }
    }

    println!("The {}th Fibonacci number is {}", nth, current_sum);
}
