#[derive(Debug)]    //* This syntax signifies an attribute, a way to attach metadata to code elements.
// Debug: The specific trait being derived, providing functionality for printing values in a debug-friendly format.
//* Enabling debug printing: By deriving Debug, you can use the {:?} formatting specifier in println! statements to print the structure or enum's contents in a readable way.


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


pub fn main(){
    // Declare a stucture
    let var = Rusty{is_rust: true, is_awesome: true};    //  The order of assigning values to the variables inside the struct doesn't matter
    println!("isRust: {}\nisAwewsome: {}", var.is_rust, var.is_awesome);
    // You can also assign values of an existing structure variable.
    let var2 = Rusty{is_rust: false, ..var};    // .. var assigns the remaining values of this variable the same as the variable used.
    println!("{:#?}", var2);    // "#" for pretty printing


    // You can also create a tuple struct, which is a tuple that is a struct.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
 
}