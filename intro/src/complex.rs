use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::cmp::{PartialEq, PartialOrd};

#[derive(Clone, Copy, Debug)]
struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    fn new(r: impl Into<f64>, i: impl Into<f64>) -> Complex {
        Complex { real: r.into(), imaginary: i.into() }
    }

    fn to_string(&self) -> String {
        if self.imaginary < 0.0 {
            String::from(format!("{}{}i", self.real, self.imaginary))
        }
        else {
            String::from(format!("{}+{}i", self.real, self.imaginary))
        }
    }

}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imaginary < 0.0 {
            write!(f, "{}{}i", self.real, self.imaginary)
        } else if self.imaginary > 0.0 {
            write!(f,"{}+{}i", self.real, self.imaginary)
        } else {
            write!(f,"{}", self.real)
        }
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary
        }
    }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary
        }
    }
}

impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        self.real == other.real && self.imaginary == other.imaginary
    }
}


pub fn run() {
    let n1 = Complex::new(2, 3);
    let n2 = Complex::new(-2, 3);
    let n3 = Complex::new(2, -3);
    let n4 = Complex::new(3, 0);
    let n5 = Complex::new(0, 3);
    let n7 = Complex::new(2, 3);
    println!("The number is {}", n1); // prints 2+3i
    println!("The number is {}", n2); // prints -2+3i
    println!("The number is {}", n3); // prints 2-3i
    println!("The number is {}", n4); // prints 3
    println!("The number is {}", n5); // prints 3i

    println!("The number is {}", n1 - n7); // prints 0
    println!("The number is {}", n1 + n2);
    println!("The number is {}", n1 - n2);

    println!(
        "The numbers {} and {} are {}",
        n1,
        n2,
        if n1 == n2 { "equal" } else { "not equal" }
    );

    // println!("The number is {}", n1 < n2);
    // println!("The number is {}", n1 <= n2);
    // println!("The number is {}", n1 > n2);
    // println!("The number is {}", n1 >= n2);
}
