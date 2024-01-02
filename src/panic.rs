use std::str;

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

pub fn main(){

    // Panic
    panic!("Oh no!!\nI panicked!!!");   // panic violently stops execution of code.

    let o1: std::option::Option<i32> = Some(128);
    o1.unwrap();

    let o2: std::option::Option<i32> = None;
    o2.unwrap();     // As "None" is found, the program panics.


    // Expect keyword that returns panic
    let s = str::from_utf8(&[195, 40]).expect("Valid utf-8");

    // Same thing can be done using match
    let melon = &[240, 159, 141, 137];
    match std::str::from_utf8(melon){
        Ok(s) => println!("{}", s),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e)
    }

    // OR
    
    // let s = str::from_utf8(melon)?;
    // println!("{}", s);
    // Doesn't work for some reason

    // same thing using "if let"
    if let Ok(s) = str::from_utf8(melon){
        println!("{}", s);
    }
}