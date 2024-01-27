use std::env;
use std::fmt::Display;
use std::path::PathBuf;


fn print_lines_from_file(f : &PathBuf) {
    let contents = std::fs::read_to_string(f).expect("Something went wrong reading the file");
    for line in contents.lines() {
        println!("{}", line);
    }
}

fn read_file<const N : usize>(f : &str) -> [[bool; N]; N]
{
    let contents = std::fs::read_to_string(f).expect("Something went wrong reading the file");
    let mut a = [[Default::default();N]; N];
    for (line, y) in contents.lines().zip(0..N) {
        // Remove leading and trailing '|' character from line
        let line = line.trim_matches('|');
        for (c, x) in line.chars().zip(0..N) {
            a[y][x] = c == 'X';
        }
    }
    a
}




fn main() {
    for i in (0..20).step_by(2) {
        println!("i is {:>2}", i);
    }

    // print current directory
    println!("{:?}", env::current_dir());

    let resourceDir = env::current_dir().unwrap().join("resources");

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

    struct SquareArray<T : Default, const N : usize>  {
        array: [[T; N]; N],
    }

    impl<T : Default + Copy, const N : usize> SquareArray<T,N> {
        fn supply() -> SquareArray<u8, N> {
            SquareArray<u8, N> {
                 [[u8; N]; N],
            };
        }
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
    let square_array = SquareArray::<bool, 10>::new();
    let aux_array = SquareArray::<u8, 10>::new();



    square_array.print();
}
