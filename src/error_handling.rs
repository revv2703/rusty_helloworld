use std::error::Error;
use std::fs;
use std::{fs::*, io, io::Read};
use std::io::ErrorKind;

pub fn main(){
    let f = File::open("Hello.txt");
    // let f = File::open("Hello.txt").expect("Failed to open Hello.txt"); // Does the same thing for the match statement thingy(prolly)
    // let f = File::open("Hello.txt").unwrap();   // does the same thing for the line below(the match statement thingy)
    let f = match f {
        Ok(file) => file,
        // Err(error) => {
        //     panic!("There was a problem opening the file: {:?}", error)
        // },

        // To handle this error i.e. create a new file and do stuff.
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("Hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };
    write("Hello.txt", "Humanity is at stack here.\n:)").expect("Could not write to file");

    //* Using closures
    let f = File::open("Hello.txt").unwrap_or_else(|error | {
        if error.kind() == ErrorKind::NotFound{
            File::create("Hello.txt").unwrap_or_else(|error | {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        }
        else{
            panic!("There was a problem opening the file: {:?}", error);
        }
    });

    let f = read_username();
    println!("{:?}", f);
}


fn read_username() -> Result<String, io::Error>{
    // let mut f = File::open("Hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    //* instead of using the commented code below, you can just use a "?" in the end.
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // match f.read_to_string(&mut s){
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // Simplifying the method further
    // let mut s = String::new();
    // File::open("Hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    // Further simplifying
    fs::read_to_string("Hello.txt");


    //* You can also use just open() followed by "?", you just need to change the function's return type to "Result<(), Box<dyn Error>>"(refer the function below)
    main_updated();

    fs::read_to_string("Hello.txt")
}

fn main_updated() -> Result<(), Box<dyn Error>>{
    let f = File::open("Hello.txt")?;
    Ok(())
}