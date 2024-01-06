use std::collections::hash_map;

use unicode_segmentation::UnicodeSegmentation;

pub fn main(){

    //* Vectors:

    let a = [1, 2, 3];

    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    let mut v2 = vec![1, 2, 3];

    let third_val = &v2[2];
    // println!("{}", third_val);

    // println!("{}", &v2[10]);    //* runtime error because the length of a vactor is dynamic hence, no compile time error unlike an array.
    
    // To handle runtime error, .get() method is used with a match statement to avoid panic.
    match v2.get(20){
        Some(val) => println!("{}", val),
        None => println!("No such element"),
    }

    // Iterating over a vector
    for i in &mut v2{
        // println!("{}", i);
        *i += 50;   // dereferencing
        println!("{}", i);  // the value of the vector changes as "i" is dereferenced.
    }


    // Using an enum to store multiple types in a vector
    #[derive(Debug)]
    enum SpreadSheetCell{
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadSheetCell::Int(5),
        SpreadSheetCell::Float(5.5),
        SpreadSheetCell::Text(String::from("Hello Vector")),
        SpreadSheetCell::Int(1) // Can store as many as you want
    ];
    for i in &row{
        println!("{:#?}", i);
    }
    
    

    //* Strings

    let mut s1 = String::new();
    let s2 = "hello rust";
    let s3 = String::from("Hello Rust");
    let s4 = s2.to_string();

    s1.push_str("Hello Rust");
    s1.push('!');
    // Can push str and char to an exisitng string, add two strings (obv).
    // let s = s1 + &s2;
    // But, the ownership of s1 will be transferred and not of s2(you know why).
    // Can also use format macro
    let s = format!("{} {}", s1, s2);   // this doesn't take ownership of the variables
    // println!("{}", s1);

    // Indexing a string
    let hello = "Hello Rust";
    // let h = &hello[0];  // this will give an error because rust doesn't know how many bytes each character takes.

    // Byte values
    for b in hello.bytes(){
        println!("{}", b);
    }
    // Scalar/chars
    for c in hello.chars(){
        println!("{}", c);
    }
    // Graphemes(in hindi, the matra is with the akshar, that's all i know)
    for g in hello.graphemes(true){
        println!("{}", g);
    }


    //* Hash maps

    use std::collections::HashMap;

    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut hash_map = HashMap::new();

    hash_map.insert(blue, 10);
    hash_map.insert(yellow, 20);

    let hash_val = String::from("Blue");
    let val = hash_map.get(&hash_val);  // Option<&i32> type because there is no guarantee of an element existing with that key.
    
    // for (key, val) in &hash_map{
    //     println!("{}: {}", key, val);
    // }

    hash_map.insert(String::from("Red"), 30);
    hash_map.insert(String::from("Green"), 40);

    hash_map.entry(String::from("Red")).or_insert(40);
    hash_map.entry(String::from("Purple")).or_insert(50);   // If purple doesn't exist add it with that value. If exists, don't do anything.

    for (key, val) in &hash_map{
        println!("{}: {}", key, val);
    }

    let mut map = HashMap::new();
    let text = "hello world wonderful world";

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;    // you dereference the variable that points to the the value of each key
    }
    println!("{:#?}", map);
}