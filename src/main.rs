use std::env;
use typenum::Unsigned;

static HELLO : &str = "HELLO";

fn main() {
    println!("Hello, world!\nI'm a rustacean");

    for i in (0..20).step_by(2) {
        println!("i is {:>2}", i);
    }

    println!("{:?}", env::current_dir());

    // print current directory
    println!("{:?}", env::current_dir());

    // define a 'squarable' trait
    trait Squarable {
        fn square(&self) -> Self;
    }

    // a macro implementing the trait for any type
    impl<T> Squarable for T
    where
        T: std::ops::Mul<Output = T> + Copy,
    {
        fn square(&self) -> Self {
            *self * *self
        }
    }

    println!("{} squared is {}", 32u32, 32u32.square());
    println!("{} squared is {}", 32u16, 32u16.square());

    // Define 2d array of integers

    struct SquareArray<T : typenum::Unsigned> {
        size: usize,
        array: [[i32; T::USIZE]; T::USIZE],
    }

    impl<T> SquareArray<T> {
        fn new(size: usize) -> SquareArray<T> {
            SquareArray {
                size,
                array: [[0; T::USIZE]; T::USIZE],
            }
        }
    }

    // Define a 'Printable' trait
    trait Printable {
        fn print(&self);
    }


    // Implement printable for square array
    impl<T> Printable for SquareArray<T>
    where
        T: typenum::Unsigned,
    {
        fn print(&self) {
            for i in 0..self.size {
                for j in 0..self.size {
                    print!("{} ", self.array[i][j]);
                }
                println!();
            }
        }
    }
}
