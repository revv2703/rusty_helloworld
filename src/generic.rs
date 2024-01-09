struct Point<T, U>{
    x: T,
    // y: T,   // This returns an error as rust expects both the variables to be of same type(same genric variable used to define both the variables)
    y: U    // To handle it, we use a different generic type "U"
}

impl<T, U> Point<T, U> {
    fn mix<V, W>(self, other: Point<V, W>) -> Point<T, W>{
        Point{
            x: self.x,
            y: other.y,
        }
        // We want x from one struct with type "T" and y from other struct that will have type "W". Therefore, return type Point<T, W>
    }
}


//* Generic types on Enums
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E>{
    Ok(T),
    Err(E),
}

pub fn main(){
    //* Generic types on functions
    let num_lst = vec![1, 2, 3, 4, 567, 6734, 53, 4, 31];

    let char_lst = vec!['a', 'b', 'c', 't', 'f', 'p'];

    let str_lst = vec!["hello", "world", "rust", "is", "fun"];

    println!("The largest number is: {}", largest_gen(num_lst));
    println!("The largest char is: {}", largest_gen(char_lst));
    println!("The largest string is: {}", largest_gen(str_lst));


    //* Generic types on structs
    let p1 = Point {x: 1, y: 2};
    let p2 = Point {x: 1.0, y: 2.0};
    let p3 = Point {x: 1, y: 2.0};

    //* Generic types on enums
    let var = match 3==2 {
        true => Some(3),
        false => None,
    };
    println!("{:?}", &var);

    //* Generic types on methods
    let p4 = Point{x: "Hello", y: 'R'};
    let p = p3.mix(p4);
    println!("p.x = {}, p.y = {}", p.x, p.y);
    
}


// We add "PartialOrd + Copy" to let rust know that our variables to be compared can actually be compared, else an error is received.
fn largest_gen<T: PartialOrd + Copy>(lst: Vec<T>) -> T {
    let mut largest = lst[0];

    for item in lst {
        if item > largest {
            largest = item;
        }
    }

    largest
}