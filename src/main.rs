use colored::Colorize;
use rand::Rng;
use std::io;
use std::str;

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
    // the compiler gives warnings when any unused variables or functions or anything are found. remove this error by inserting "_" at the beginning of the variable.


    // if num == stdin{
    //     println!("\nCorrect Guess. :)\n\nThe number is: {}", stdin);
    // }
    // else{
    //     println!("\nIncorrect guess. :(\n\nYou guessed: {}\nThe number was: {}\n", stdin, num)
    // }

    
    // Match statement:

    let is_equal = stdin == _num;
    match is_equal {
        true => println!("\nCorrect Guess. :)\n\nThe number is: {}", stdin.to_string().green()),
        false => println!("\nIncorrect guess. :(\n\nYou guessed: {}\nThe number was: {}\n", stdin, _num.to_string().red()),
        _ => println!("Some error that can never be encountered.")
    }
    // The match has to be exhaustive, i.e. the value provided must match one value, to handle no match, "_" is used
}

struct Rusty{   // Start a struct name with capital
    is_rust: bool,
    is_awesome: bool
}

impl Rusty{
    fn boo(){
        println!("Boo!");
    }
    fn check_rust(&self){
        if self.is_rust == true {println!("Is Rust");}  // Need parenthesis
    }
}


// Another implementation of panic
enum Option<T> {
    None,
    Some(T)
}

impl<T> Option<T> {
    fn unwrap(self) -> T {
        match self{
            Self::Some(t) => t,
            Self::None => panic!("Panic!!")
        }
    }
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}




fn main() {
    // println!("Hello, world!\n");

    // Data types:

    // Scalar types:

    // -Integers
    // let a = 98_322; // decimal
    // let b = 0xff;   // hex
    // let c = 0o77;   // octal
    // let d = 0b1111_0000;    // binary
    // let e = b'A';   // byte(u8 only)
    // if overflow, rust panicks, but in production (var%(int_length)), e.g. let f = 257 becomes 1


    // -Floating points
    // let f = 2.0;
    // let g: f32 = 3.0;


    // -Boolean
    // -Char


    // Compound types:

    // -Tuple
    // let tup = ("hello", 19, true);
    // let (string, num, truth) = tup; // To extract value from a tuple
    // let string = tup.0;  // another way


    // -List
    // let lst = [100, 200, 300];
    // let byte = [0;8];   // this generates 8 "0" values in a list


    // Control flow
    // - if, if else, else

    // - loop
    // let mut count = 0;
    // let result = loop{
    //     count+=1;
    //     if count == 5{
    //         break count;
    //     }
    // };
    // print!("{}", result);
    // Loop can also be assigned to a variable that returns a value

    // - while

    // - for
    // let a = [1,2,3,4,5];
    // for el in a.iter(){println!("The element is: {}", el)};
    // for i in 1..5{println!("The number is: {}", i)} // same as python, 1 to 4 will be printed



    // Line comment
    /*
        Block comment
     */




    // let tup = ("hello", 19, true);
    // println!("{:?}", tup);
    // dbg!(tup); // Used for dirty debugging -> don't know what it means... meh

    // Random number game(pretty useless this is)
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

    // mut  ->  The "mut" keyword allows for the user to change variable values after its assignment. Even vectors.

    // let mut v1 = Vec::new();    // Returns error as there is no type for the variable.
    // v1.push(1); // Doesn't return any error after this line of code.
    // println!("{}", v1[0]);

    // let mut v1 = vec![true, false];
    // println!("{}", v1[0]);
    // Can reinitialize variable with the same name and/or different type.

    // Panic
    // panic!("Oh no!!\nI panicked!!!");   // panic violently stops execution of code.

    // let o1: std::option::Option<i32> = Some(128);
    // o1.unwrap();

    // let o2: std::option::Option<i32> = None;
    // o2.unwrap();     // As "None" is found, the program panics.


    // Expect keyword that returns panic
    // let s = str::from_utf8(&[195, 40]).expect("Valid utf-8");

    // Same thing can be done using match
    // let melon = &[240, 159, 141, 137];
    // match std::str::from_utf8(melon){
        // Ok(s) => println!("{}", s),
        // Err(e) => panic!("Invalid UTF-8 sequence: {}", e)
    // }

    // OR
    
    // let s = str::from_utf8(melon)?;
    // println!("{}", s);
    // Doesn't work for some reason

    // same thing using "if let"
    // if let Ok(s) = str::from_utf8(melon){
        // println!("{}", s);
    // }


    // Iterators
    // let natural_nos = 1..;  // 1 or greater than 1
    // println!("{:?}", natural_nos);
    // let ans = (0..).contains(&100);
    // println!("{}", ans);
    // (..=20).contains(&20);  // <=20
    // (3..6).contains(&4);    // 3, 4, 5
    // let num1 = 1;let num2 = 5;
    // println!("{}", (num1..num2).contains(&3));   // can do the same with variables

    // for loop
    // for i in vec![1, 2, 3]{
    //     println!("The number is: {}", i);
    // }
    // for c in "rust".chars(){
    //     println!("The character is: {}", c);
    // }
    // for c in "SuRPRISE INbOUND".chars().filter(|c| c.is_lowercase()).flat_map(|c| c.to_uppercase()){
    //     print!("{}", c);
    // }
}
