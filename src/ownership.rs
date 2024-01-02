pub fn ownership_fn(){
    //* Each value has a variable that's called its owner. Only one owner at a time. After the owner goes out of scope, the value is also dropped.
    //* The scope of a variable ends when used for the last time.

    let x = 5;
    let y = x;
    println!("{}", y);  // copy

    let s1 = String::from("hello world!");
    let s2 = s1;

    // Integers are copied and strings are moved.

    // As we know only one owner can exist for a value, and when assigning s1 to s2, the value of s1 is assigned to s2 and the s1 goes out of scope.
    // println!("{}", s1);  // This will throw an error
    // println!("Move: {}", s2);  // This will work fine // Move (not shallow copy)

    // to clone
    let s3 = s2.clone();
    // println!("Clone: {} | {}", s3, s2);

    // take_ownership(s3); // can fix it by replacing s3 with s3.clone()
    // print!("{}", s3);   // value borrowed here after move
    
    // we can also do this
    let s3 = give_ownership(s3);
    // println!("{}", s3);

    // Take and give ownership of a variable
    let s4 = give_ownership(s3);
    // println!("{}", s3);  // throws error


    //* We can also use a variable's value and not take ownership from it by passing a reference of it in a function.
    let s5 = take_ownership_ref(&s4);


    // Change the value of a variable using a function
    let mut s6 = String::from("Hello World!");
    // change(&mut s6);


    // You can only pass mutable reference of a variable once.
    // let r1 = &mut s6;
    let r1 = & s6;
    // let r2 = &mut s6;   // returns error only when the variable r2 is used.
    // print!("{} | {}", r1, r2);
    // to fix this, replace the mutable references with immutable references.


    // Also you cannot have a mutable reference if you have even a single immutable reference. This can be avoided if the mutable reference is declared after the scope of the variables using the referenced variable.
    let r3 = &mut s6;
    // println!("{}", r3);


    // let invalid_reference = dangle();    // Invalid reference to a value


}



// fn dangle() -> &String{ // The value returned is a reference to the variable inside the function that will be dropped after the execution of the function and hence the value returned will point to nothing, hence invalid.
//     let s = String::from("Hello World!");
//     &s
// }

fn change(str:&mut String){
    print!("The value of the variable is changed from: {}", str);
    str.push_str(" This is great!");
    println!("\nto: {}", str);
}

fn take_ownership(str:String) -> String{
    println!("This also moves the value of the variable to the function: {}", str);
    str
}
fn take_ownership_ref(str: &String){
    println!("The value of the variable is passed to the function as a reference: {}", str);
}

fn give_ownership(str:String) -> String{
    println!("The ownership's value: {}", str);
    str
}

fn take_and_give_ownership(str:String) -> String{
    println!("This also moves the value of the variable to another variable: {}", str);
    str
}