use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome, I'll Guess the number you're thinking of.");
    println!("Enter your number: ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read");
    println!("Using sophisticated AI models to get into your brain......");
    println!("Easy, your guess is {guess}");
    println!("Now, it's my turn.");
    let secret = rand::thread_rng().gen_range(1..=101);
    println!("Secret number is {secret}");
    loop {
        println!("Enter your guess: ");
        guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your guess is {guess}");
        match guess.cmp(&secret){
            Ordering::Less => println!("low, like your IQ!"),
            Ordering::Equal => {
                println!("Wow..., you guessed right"); 
                break;
            },
            Ordering::Greater => println!("Too high, like your ego"),
        }
    }
}
