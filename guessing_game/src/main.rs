use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!", "");
    let secret_number = rand::thread_rng().gen_raange(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "solve" {
            auto_solve(secret_number);
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

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

fn auto_solve(num: u32) {
    let mut left = 1;
    let mut right = 100;
    loop {
        let middle = (left + right) / 2;
        println!("Guess: {}", middle);
        match middle.cmp(&num) {
            Ordering::Less => {
                left = middle;
            }
            Ordering::Greater => {
                right = middle;
            }
            Ordering::Equal => {
                println!("The secret number is: {}!", middle);
                break;
            }
        };
    }
}
