use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    println!("You guessed: {} ", secret_number);
}
