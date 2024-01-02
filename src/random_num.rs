use rand::Rng;
use std::{io, cmp::Ordering};
use colored::Colorize;

fn gen_random() -> u32 {
    return rand::thread_rng().gen_range(0..10);

    // rand::thread_rng().gen_range(0..10)  // can return variables by removing return keyword and the ";"
}

fn take_input() -> u32 {
    let mut input = String::new();
    println!("Enter an integer: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input: u32 = input.trim().parse().expect("Invalid input. Please type a number.");

    input
}

fn random_num() {

    let _num: u32 = gen_random();

    //* Typical approach
    // for i in 0..3{
    //     let stdin = take_input();
    //     if stdin > _num{
    //         println!("Your guess {} is lower than the number.", stdin);
    //     }
    //     else if stdin < _num{
    //         println!("Your guess {} is higher than the number.", stdin);
    //     }
    // }

    //* Typical approach
    // if num == stdin{
    //     println!("\nCorrect Guess. :)\n\nThe number is: {}", stdin);
    // }
    // else{
    //     println!("\nIncorrect guess. :(\n\nYou guessed: {}\nThe number was: {}\n", stdin, num)
    // }

    

    //* Match statement:

    for _i in 0..3{
        let stdin = take_input();
        match stdin.cmp(&_num){
            Ordering::Less => println!("{} {} {}", "Your guess".red(), stdin, "is lower than the number.".red()),
            Ordering::Greater => println!("{} {} {}", "Your guess".red(), stdin, "is higher than the number.".red()),
            Ordering::Equal => println!("{} {} {}", "Your guess".green(), stdin, "is correct.\n\nU WIN!!".green())
        }
    }


    //* Match statement(typical):
    // let is_equal = stdin == _num;
    // match is_equal {
    //     true => println!("\nCorrect Guess. :)\n\nThe number is: {}", stdin.to_string().green()),
    //     false => println!("\nIncorrect guess. :(\n\nYou guessed: {}\nThe number was: {}\n", stdin, _num.to_string().red()),
    //     _ => println!("Some error that can never be encountered.")
    // }
    // The match has to be exhaustive, i.e. the value provided must match one value, to handle no match, "_" is used
}

pub fn main(){

    // Random number game(pretty useless this is)
    random_num();
}