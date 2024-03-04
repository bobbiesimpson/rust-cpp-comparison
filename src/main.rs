use std::io;
use rand::Rng;

fn main() {

    let lower_bound = 1;
    let upper_bound = 100;
    let n = rand::thread_rng().gen_range(lower_bound..=upper_bound);
    let mut int_guess = -1;

    println!("Guess the number between {lower_bound} and {upper_bound}");
    while int_guess != n {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read string");
        int_guess = guess.trim().parse::<i32>().unwrap();

        if int_guess > n {
            println!("Too high");
        }
        else if int_guess < n {
            println!("Too low");
        }
        else {
            println!("Bang on!")
        }
    }
}
