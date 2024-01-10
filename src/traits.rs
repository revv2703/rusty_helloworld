//* Trait is a powerful tool for defining functionality that different types can share. It acts like a blueprint for behavior, outlining the methods and associated types that any type implementing the trait must provide.
/*
    A definition of shared behavior for different types.
    Specifies methods and associated types (optional) that implementing types must define.
    Plays a crucial role in achieving abstraction and polymorphism in Rust.
*/

use std::fmt::{Display, Debug};


pub struct NewsArticle{
    pub author: String,
    pub headline: String,
    pub content: String
}

pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

pub trait Summary{
    fn summarize(&self) -> String{
        format!("Read more from {}...", self.summarize_author())    // This is default implementation, if the implementation doesn't have a function for the mentioned function, this function's output is returned.
    }
    fn summarize_author(&self) -> String;
}

impl Summary for NewsArticle{
    // Using the default implementation
    // fn summarize(&self) -> String {
    //     format!("{}, by {}", self.headline, self.author)
    // }
    // Now for every not defualt implementation, You have to define an implementation
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

impl Summary for Tweet{
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Uisng traits as parameters
// pub fn notify(item: &impl Summary){
//     println!("Breaking news! \n{}", item.summarize());
// }

// Using traits as parameters with trait bounds
// pub fn notify<T: Summary>(item: &T){    // Generic type but only from trait Summary
//     println!("Breaking news! \n{}", item.summarize());
// }

// Using traits as parameters with trait bounds and multiple parameters
// pub fn notify(item1: &impl Summary, item2: &impl Summary){    // Cannot make sure if both the parameters have same type
//     println!("Breaking news! \n{}\n{}", item1.summarize(), item2.summarize());
// }

// Using traits as parameters with trait bounds and multiple parameters and trait bounds
// Can specifiy multiple traits using "+"
pub fn notify<T: Summary>(item1: &T, item2: &T){    // Can make sure both the parameters have the same type
    println!("Breaking news! \n{}\n{}", item1.summarize(), item2.summarize());
}


// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U){   // Hinders readability
// }
// To fix readability issues,
fn some_function<T, U>(t: &T, u: &U)
    where T: Display + Clone,
    U: Clone + Debug{
    // ...
}


// Returning types that implement traits
fn returns_summarizable() -> impl Summary{  // returns a value that implements Summary
    Tweet{
        username: String::from("Revv"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    }
}


struct Pair<T>{
    x: T,
    y: T
}

// To conditionally implement trait methods
impl<T> Pair<T>{
    fn new(x: T, y: T) -> Self{
        Self{x, y}
    }
}
impl<T: Display + PartialOrd> Pair<T>{
    fn cmp_display(&self){
        if self.x >= self.y{
            println!("The largest member is x = {}", self.x);
        }
        else{
            println!("The largest member is y = {}", self.y);
        }
    }
}


pub fn main(){
    let tweet = Tweet{
        username: String::from("Revv"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };

    let article = NewsArticle{
        author: String::from("Revv"),
        headline: String::from("This is a headline"),
        content: String::from("This is the content")
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new article: {}", article.summarize());

    notify(&article, &article);

    println!("{:?}", returns_summarizable().summarize());

    let var_int = Pair{x: 1, y: 2};
    var_int.cmp_display();  // The var_int variable is assigned the second impl because the values are of type Display + PartialOrd, therefore cannot access new method.
}