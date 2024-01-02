#[derive(Debug)]    //* This syntax signifies an attribute, a way to attach metadata to code elements.
// Debug: The specific trait being derived, providing functionality for printing values in a debug-friendly format.
//* Enabling debug printing: By deriving Debug, you can use the {:?} formatting specifier in println! statements to print the structure or enum's contents in a readable way.


struct Rusty{   // Start a struct name with capital
    is_rust: bool,
    is_awesome: bool
}

// You can also create a struct with methods.
impl Rusty{
    fn boo(){
        println!("Boo!");
    }
    fn check_rust(&self){
        if self.is_rust == true {println!("Is Rust");}  // Need parenthesis
    }
}


#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle{
    fn calc(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

// Can create multiple implementation blocks for the same struct
impl Rectangle{
    fn square(size: u32) -> Rectangle{
        Rectangle{width: size, height: size}
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
    let black = Color(0, 0, 0);

    
    // You can also create a unit struct, which is a struct with no fields.

    let rect = Rectangle{width: 40, height: 50};
    println!("The area of rectangle: {}", rect.calc());  // "&" because u don't want ownership of the value


    let rect1 = Rectangle{
        width: 10,
        height: 40
    };
    println!("Rectangle can hold rect1: {}", rect.can_hold(&rect1));


    let sq = Rectangle::square(10);
    println!("Square: {:#?}", sq);
 
}