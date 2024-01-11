use std::{result, fmt::Display};

// The lifetimes of the parameters passed in a function are called "input lifetimes" and that of output values are called "output lifetimes"


struct ImportantExcerpt<'a>{
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a>{
fn announce_and_return_part(&self, announcement: &str) -> &str{
        println!("Attention please: {}", announcement);
        self.part   // output lifetime same as "self"
    }
}

pub fn main(){
    // let r;
    // {
    //     let x = 5;
    //     r = &x; // X DOES NOT LIVE LONG ENOUGH
    //     r has a dangling reference
    // }
    // let x = 5;
    // let r = &x; // and not here(same scope)

    // println!("r: {}", r);


    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let res = longest(string1.as_str(), string2.as_str());
    println!("The longest string is: {}", res);


    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());   // string2 goes out of scope here, but string1 is still valid. therefore, result too goes out of scope
    // }
    // println!("The longest string is: {}", result);

    // let res = longest_without_annotation(string1.as_str(), string2.as_str());
    // println!("{}", res);

    // There are scenarios where the compiler can deterministically infer the lifetime annotations
    //? Three rules:
    //* 1. Each parameter that is a reference gets its own lifetime parameter
    //* 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
    //* 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters


    let novel = String::from("Call me Revv. Some days ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a sentence");
    let i = ImportantExcerpt{
        part: first_sentence
    };


    //* Static lifetime -> The reference can live as long as the duration of the entire program
    // All string literals have static lifetimes
    
}

// fn longest(x: &str, y: &str) -> &str {   // returns error because the compiler doesn't know when the lifetime of the variable referenced ends, and if the value returned is used after that...
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// i32      a reference
// &'a i32  a reference with an explicit lifetime
// &'a mut i32  a mutable reference with an explicit lifetime

// To fix this we introduce a "Generic lifetime annotation", can be anything after "'"(single word) but preferably kept a lowercase character
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{  // this makes sure that every value returned will have the same lifetime as "'a", which will have the smallest lifetime of all input parameters
    if x.len() > y.len(){
        x
    } else {
        y
    }
}
// The reference that is returned by a function must always be a reference to the given parameters because one can never return any variable created within the function
// The referenced return value must be tied to any one parameter's lifetime

// fn longest_without_annotation<'b>(x: &str, y: &str) -> &'b str{
//     let result = String::from("Hello Rust");
//     result.as_str()
// Can just return String instead of a string reference, will work
// }

// To sum it all
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, announce: T) -> &'a str where T:Display{
    println!("Announcement! {}", announce);
    if x.len() > y.len(){
        x
    } else {
        y
    }
}