use std::env;
use std::fmt::Display;

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

    struct SquareArray<T : Default, const N : usize> {
        array: [[T; N]; N],
    }

    impl<T : Default + Copy, const N : usize> SquareArray<T,N> {
        fn new() -> Self {
            SquareArray {
                // initialize array
                array: [[T::default() ; N]; N],
            }
        }
    }
    // Print current directory
    println!("Current directory is {:?}", env::current_dir());

    // Define a 'Printable' trait
    trait Printable {
        fn print(&self);
    }

    // Implement printable for square array
    impl<T : Display + Default,const N : usize> Printable for SquareArray<T,N>
    {
        fn print(&self) {
            for i in 0..N {
                for j in 0..N {
                    print!("{} ", self.array[i][j]);
                }
                println!();
            }
        }
    }

    // Define a square array of size 10
    let square_array = SquareArray::<u8, 10>::new();

    square_array.print();
}
