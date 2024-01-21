mod structure;
mod iters;

pub fn adds_two(a: i32) -> i32{
    println!("The value passed: {}", a);    // The print statement only shows up for failed tests. To sohow the print statement for passed tests also, pass "cargo test -- --show-output"
    a + 2
}

#[cfg(test)]
mod tests {
    use crate::{structure::Rectangle, iters::{Iterator, self}};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    // #[test]
    // fn failed_test() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_fit_smaller(){
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger), "The first rectangle: {:#?} cannot store the other one: {:#?}", smaller, larger);
        // If one fails, the test fails
    }

    #[test]
    fn it_adds_two(){
        assert_eq!(4, super::adds_two(2), "Number given + 2 != 4"); // custom error message
    }

    #[test]
    #[should_panic(expected = "Make this test fail")]
    fn it_should_panic(){
        panic!("Make this test fail");
        // panic!("Make this fail");    the test will fail even after the "should panic", because the expected panic message is: "Make this test fail", any other panic message returned counts as a fail
    }

    // Run specific tests using "cargo test test_name"
    // If multiple functions run with same subname, use "cargo test common_subname"
    // Run tests from different modules using "cargo test module_name::"


    #[test]
    #[ignore]
    fn expensive_test(){
        // This test will run only when "cargo test -- --ignored" is run
        // To run only ignored ones, and run them sequentially, use "cargo test -- --ignored --test-threads=1"
        // To run only ignored ones, and run them sequentially, and show output, use "cargo test -- --ignored --test-threads=1 --show-output"
        assert!(true);
    }

    #[test]
    fn Iterator_demons(){
        let v = vec![1, 2, 3];

        let mut v_iter = v.iter();
        // let mut v_iter = v.iter_mut();  // To get mutable references
        // let mut v_into_iter = v.into_iter();    // To get ownership of the vector and return owned values, to execute this -> remove "&".

        assert_eq!(v_iter.next(), Some(&1));
        assert_eq!(v_iter.next(), Some(&2));
        assert_eq!(v_iter.next(), Some(&3));
        assert_eq!(v_iter.next(), None);    // None is returned when the iterator reaches the end of the collection
    }

    #[test]
    fn iterator_sum(){
        let v = vec![1, 2, 3];

        let v_iter = v.iter();

        let total: i32 = v_iter.sum();  // sum() is a method on the Iterator trait, and can be used only on iterators. Runs till the end.
        assert_eq!(total, 6);
    }

    #[test]
    fn filter_by_size(){
        use super::*;
        let shoes = vec![
            iters::Shoe{size: 10, style: String::from("sneaker")},
            iters::Shoe{size: 13, style: String::from("sandal")},
            iters::Shoe{size: 10, style: String::from("boot")}
        ];

        let in_my_size = iters::shoes_in_my_size(shoes, 10);
        assert_eq!(in_my_size, vec![
            iters::Shoe{size: 10, style: String::from("sneaker")},
            iters::Shoe{size: 10, style: String::from("boot")}
        ])
    }

    #[test]
    fn counter_iterator(){
        let mut counter = iters::Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));

        assert_eq!(counter.next(), None);
    }

    // #[test]
    // fn counter_iterator_sum(){
    //     let sum: u32 = iters::Counter::new().zip(iters::Counter::new().skip(1))
    //         .map(|(a, b)| a * b)
    //         .filter(|x| x % 3 == 0)
    //         .sum();
        
    //     assert_eq!(18, sum);
    // }
    // Messed up test(to be fixed)
}