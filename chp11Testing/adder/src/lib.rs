#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(val: i32) -> i32 {
    val + 2
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn explore() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        }
        else {
            Err("2 + 2 is not 4".to_string())
        }
    }

    #[test]
    #[should_panic]
    fn another() {
        panic!("make fail")
    }

    #[test]
    fn adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn hold_smaller() {
        let larger = Rectangle {
            width: 5,
            height: 10
        };
        let smaller = Rectangle {
            width: 2,
            height: 4,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn not_hold() {
        let smaller = Rectangle {
            width: 2,
            height: 4,
        };
        let larger = Rectangle {
            width: 5,
            height: 10,
        };
        assert!(!smaller.can_hold(&larger))
    }
}

