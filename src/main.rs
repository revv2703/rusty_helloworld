use rand::Rng;
use std::io;

fn gen_random() -> i32 {
    return rand::thread_rng().gen_range(0..10);

    // rand::thread_rng().gen_range(0..10)  // can return variables by removing return keyword and the ";"
}

fn take_input() -> i32 {
    let mut input = String::new();
    println!("Enter an integer: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let num: i32 = input.trim().parse().expect("Invalid input");

    return num;
}

fn random_num() {
    let stdin = take_input();
    let num = gen_random();
    if num == stdin{
        println!("\nCorrect Guess. :)\n\nThe number is: {}", stdin);
    }
    else{
        println!("\nIncorrect guess. :(\n\nYou guessed: {}\nThe number was: {}\n", stdin, num)
    }
}

fn main() {
    // println!("Hello, world!\n");
    // let tup = ("hello", 19, true); // Tuple
    // println!("{:?}", tup); // How you print a variable
    // dbg!(tup); // Used for dirty debugging -> don't know what it means... meh
    random_num();
}
