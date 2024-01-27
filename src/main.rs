use std::env;

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
}
