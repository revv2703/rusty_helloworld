pub mod hello{
    pub fn hello(){
        println!("Hello, world!");
    }
}

pub fn main(){
    println!("Hello, world!\n");

    // the compiler gives warnings when any unused variables or functions or anything are found. remove this error by inserting "_" at the beginning of the variable.


    //* Data types:


    // Scalar types:

    // -Integers
    let a = 98_322; // decimal
    let b = 0xff;   // hex
    let c = 0o77;   // octal
    let d = 0b1111_0000;    // binary
    let e = b'A';   // byte(u8 only)
    // if overflow, rust panicks, but in production (var%(int_length)), e.g. let f = 257 becomes 1


    // -Floating points
    let f = 2.0;
    let g: f32 = 3.0;


    // -Boolean
    // -Char


    // Compound types:

    // -Tuple
    let tup = ("hello", 19, true);
    let (string, num, truth) = tup; // To extract value from a tuple
    let string = tup.0;  // another way


    // -List
    let lst = [100, 200, 300];
    let byte = [0;8];   // this generates 8 "0" values in a list


    // Control flow
    // - if, if else, else

    // - loop
    let mut count = 0;
    let result = loop{
        count+=1;
        if count == 5{
            break count;
        }
    };
    print!("{}", result);
    // Loop can also be assigned to a variable that returns a value

    // - while
    let mut x = 0;
    while x < 5{
        println!("count is: {}", x);
        x+=1;
    }

    // - for
    let a = [1,2,3,4,5];
    for el in a.iter(){println!("The element is: {}", el)};
    for i in 1..5{println!("The number is: {}", i)} // same as python, 1 to 4 will be printed

    // for i in vec![1, 2, 3]{
    //     println!("The number is: {}", i);
    // }
    // for c in "rust".chars(){
    //     println!("The character is: {}", c);
    // }
    // for c in "SuRPRISE INbOUND".chars().filter(|c| c.is_lowercase()).flat_map(|c| c.to_uppercase()){
    //     print!("{}", c);
    // }


    // Line comment

    /*
        Block comment
    */


    let tup = ("hello", 19, true);
    println!("{:?}", tup);
    dbg!(tup); // Used for dirty debugging -> don't know what it means... meh



    let x = {
        let y = 10;
        let z = 15;
        y + z
    };
    println!("{}", x);  // Cannot access y and z variables
    // loop {println!("loop exec");} // more like while(true) loop


    let var = "helloworld";
    println!("\nLength of the string \"{}\" is: {}\n", var, var.len());
    println!("{}", tup.0.len());  // can access elements on i index by using "." followed by the index.



    // mut  ->  The "mut" keyword allows for the user to change variable values after its assignment. Even vectors.

    let mut v1 = Vec::new();    // Returns error as there is no type for the variable.
    v1.push(1); // Doesn't return any error after this line of code.
    println!("{}", v1[0]);

    let mut v1 = vec![true, false];
    println!("{}", v1[0]);


    // Can reinitialize variable with the same name and/or different type called "Shadowing".


    // Iterators
    let natural_nos = 1..;  // 1 or greater than 1
    println!("{:?}", natural_nos);
    let ans = (0..).contains(&100);
    println!("{}", ans);
    (..=20).contains(&20);  // <=20
    (3..6).contains(&4);    // 3, 4, 5
    let num1 = 1;let num2 = 5;
    println!("{}", (num1..num2).contains(&3));   // can do the same with variables
}