use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let lower_bound = 1;
    let upper_bound = 100;
    let n = rand::thread_rng().gen_range(lower_bound..=upper_bound);

    println!("Guess the number between {lower_bound} and {upper_bound}");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read string");
        let int_guess = guess.trim().parse::<i32>().unwrap();

        match int_guess.cmp(&n) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("Bang on!");
                break;  // break loop
            }
        }
    }
}
