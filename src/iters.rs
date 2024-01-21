pub trait Iterator{
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

#[derive(PartialEq, Debug)]
pub struct Shoe{
    pub size: u32,
    pub style: String
}

pub fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe>{
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

// Creating your own iterator
pub struct Counter{
    count: u32
}

impl Counter{
    pub fn new() -> Counter{
        Counter{count: 0}
    }
}

impl Iterator for Counter{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item>{
        if self.count < 5{
            self.count += 1;
            Some(self.count)
        }else{
            None
        }
    }
}


pub fn main(){
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    for val in v1_iter{
        println!("Got: {}", val);
    }
    
    // Refer lib.rs for tests(Iterator_demons, iterator_sum)

    let v = vec![1, 2, 3];
    let v_new : Vec<_> = v.iter().map(|x| x + 1).collect();

    assert_eq!(v_new, vec![2, 3, 4]);

    // Another test(filter_by_size)
}