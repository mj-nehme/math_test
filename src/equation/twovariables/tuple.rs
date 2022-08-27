use std::fmt;

// Tuple struct is implemented (instead of a tuple (i32, i32))
// because the tuple (i32, i32) can't implement the Disply trait
#[derive(Debug, PartialEq)]
pub struct Tuple {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x = {}, y = {}", self.x, self.y)
    }
}
