use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
     loop{

    println!("Welcome to the guessing game!");

    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(0..100);

    // println!("The random number generated is {random_number}");

    println!("Enter your number guess");
    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read the input");

    let guess : u32 =match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("Your Guess is {guess}");

    match guess.cmp(&random_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
        println!("You win!")
        break;
}
    }
}


}
