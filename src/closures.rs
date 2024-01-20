use std::thread;
use std::time::Duration;


fn simulated_expensive_calc(intensity: u32) -> u32{
    println!("Calculating...");
    thread::sleep(Duration::from_secs(3));
    intensity
}

fn generate_workout(intensity: u32, random_num: u32){
    // let expensive_res = simulated_expensive_calc(intensity);
    let mut cached_res = Cacher::new(|num| {
        println!("Calculating...");
        thread::sleep(Duration::from_secs(3));
        num
    });

    if intensity < 25{
        println!("Today, do {} pushups!", cached_res.value(intensity));
        println!("Next, do {} situps!", cached_res.value(intensity));
    }
    else{
        if random_num == 3{
            println!("Take a break today!");
        }
        else{
            println!("Run for {} minutes today", cached_res.value(intensity));
        }
    }
}

struct Cacher<T> where T: Fn(u32) -> u32,{
    calculation: T,
    value: Option<u32>
}

// FnOnce(trait) -> Takes the owndership of the variable
// FnMut(trait) -> Takes the mutable reference of the variable
// Fn(trait) -> Takes the immutable reference of the variable
// The Fn keyword below
impl<T> Cacher<T> where T:Fn(u32) -> u32,{
    fn new(calculation: T) -> Cacher<T>{
        Cacher{
            calculation,
            value: None
        }
    }
    fn value(&mut self, arg: u32) -> u32{
        match self.value{
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn main(){
    // println!("{:?}", simulated_expensive_calc(5));
    // generate_workout(10, 7);
    // let x = 4;
    // fn equal_to_x(z: i32) -> bool{
    //     z == x  // Returns error because a function cannot capture variables withinng the scope not passed into it, but closures can.
    // }
    
    // let equal_to_x = |z| z == x;

    // let y = 4;

    // assert!(equal_to_x(y));

    let x = vec![1,2,3];
    let equal_to_x  = |z| z == x;
    // let equal_to_x = move |z| z == x;    // To force the closure to take ownership of the variable, we use "move".
    println!("Cannot use x here: {:?}", x);

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));

}
