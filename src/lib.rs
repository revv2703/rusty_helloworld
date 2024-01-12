mod structure;

pub fn adds_two(a: i32) -> i32{
    a + 2
}

#[cfg(test)]
mod tests {
    use crate::structure::Rectangle;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn failed_test() {
        panic!("Make this test fail");
    }

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
        assert!(smaller.can_hold(&larger), "The first rectangle: {:#?} cannot store the other one: {:#?}", smaller, larger);
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
}