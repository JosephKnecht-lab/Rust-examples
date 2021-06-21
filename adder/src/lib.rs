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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(2 + 2, 4);
    }


    #[test]
    fn another() {
        // panic!("test fails");
    }

    #[test]
    fn large_can_hold_small(){
        let larger = Rectangle {
            width: 7,
            height: 8,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cant_hold_larger() {
        let larger = Rectangle {
            width: 7,
            height: 8,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
