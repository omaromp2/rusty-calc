// src/calc.rs

// A simple Calculator struct that holds two integers.
pub struct Calc {
    pub x: i32,
    pub y: i32,
}

impl Calc {
    // Creates a new `Calculator` with the given x and y values.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // Returns the sum of x and y.
    pub fn add(&self) -> i32 {
        self.x + self.y
    }

    // Returns the difference of x minus y.
    pub fn subtract(&self) -> i32 {
        self.x - self.y
    }

    // Returns the product of x and y.
    pub fn multiply(&self) -> i32 {
        self.x * self.y
    }

    // Returns the quotient of x / y if y != 0, otherwise `None`.
    pub fn divide(&self) -> Option<i32> {
        if self.y == 0 {
            None
        } else {
            Some(self.x / self.y)
        }
    }
}

