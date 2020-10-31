use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the guessing game!");
    println!("Guess the number");

    let secret_number = rand::thread_rng()
        .gen_range(1,101);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Something broke");
        
        let guess: u32 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
        println!("Your guess is : {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            },
            Ordering::Greater => println!("Too Big!")
        }
    }
}

