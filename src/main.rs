mod stub;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let result = std::panic::catch_unwind(|| {
        guess();
    });
    if result.is_err() {
        eprintln!("error occured: {:?}", result);
    }
}

fn guess() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().unwrap();

        println!("You guessed: {guess}");
        assert!(guess <= 100);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
