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
    let _num = gen_random();
    // the compiler gives warnings when any unused variables are found. remove this error by inserting "_" at the beginning of the variable.


    // if num == stdin{
    //     println!("\nCorrect Guess. :)\n\nThe number is: {}", stdin);
    // }
    // else{
    //     println!("\nIncorrect guess. :(\n\nYou guessed: {}\nThe number was: {}\n", stdin, num)
    // }

    
    // Match statement:

    let is_equal = stdin == _num;
    match is_equal {
        true => println!("\nCorrect Guess. :)\n\nThe number is: {}", stdin),
        false => println!("\nIncorrect guess. :(\n\nYou guessed: {}\nThe number was: {}\n", stdin, _num),
        _ => println!("Some error that can never be encountered.")
    }
    // The match has to be exhaustive, i.e. the value provided must match one value, to handle no match, "_" is used
}

struct Rusty{   // Start a struct name with capital
    is_rust: bool,
    is_awesome: bool
}

fn main() {
    // println!("Hello, world!\n");
    // let tup = ("hello", 19, true); // Tuple
    // println!("{:?}", tup); // How you print a variable
    // dbg!(tup); // Used for dirty debugging -> don't know what it means... meh
    // random_num();

    // let x = {
    //     let y = 10;
    //     let z = 15;
    //     y + z
    // };
    // println!("{}", x);  // Cannot access y and z variables
    // loop {println!("loop exec");} // more like while(true) loop


    // let var = "helloworld";
    // println!("\nLength of the string \"{}\" is: {}\n", var, var.len());
    // println!("{}", tup.0.len());  // can access elements on i index by using "." followed by the index.

    // Declare a stucture
    // let var = Rusty{is_rust: true, is_awesome: true};    //  The order of assigning values to the variables inside the struct doesn't matter
    // println!("isRust: {}\nisAwewsome: {}", var.is_rust, var.is_awesome);


}
