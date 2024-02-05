use std::env;
//use std::fmt::Display;
use std::path::PathBuf;

const N: usize = 9;

#[derive(Clone)]
struct SquareArray<T: Default + Repr> {
    array: [[T; N]; N],
}

trait DumpableWithMax {
    fn dunmpWithMax(&self, maxData: (i32, i32, i32));
}

impl<T: Default + Copy + Repr> SquareArray<T> {
    fn create_aux(&self) -> SquareArray<i32> {
        SquareArray::<i32> {
            array: [[Default::default(); N]; N],
        }
    }

    fn print(&self) {
        for i in 0..N {
            for j in 0..N {
                print!("{} ", self.array[i][j].repr());
            }
            println!();
        }
    }


    fn new() -> Self {
        SquareArray {
            // initialize array
            array: [[T::default(); N]; N],
        }
    }
}

impl DumpableWithMax for SquareArray<i32> {
    fn dunmpWithMax(&self, maxData: (i32,i32,i32)) {
        let (x, y, num) = maxData;
        let mut ret = self.clone();
        for yoffset  in y - num + 1..=y {
            for xoffset in x - num + 1..=x {
                ret.array[yoffset as usize][xoffset as usize] -= 1;
            }
        }
        self.print();
    }
}
// Print current directory
//println!("Current directory is {:?}", env::current_dir());

trait Repr {
    fn repr(&self) -> String;
}

impl Repr for bool {
    fn repr(&self) -> String {
        if *self {
            "X".to_string()
        } else {
            " ".to_string()
        }
    }
}

impl Repr for usize {
    fn repr(&self) -> String {
        return format!("{}", self);
    }
}

impl Repr for i32 {
    fn repr(&self) -> String {
        return if *self == -1 {
            "X".to_string()
        } else {
            " ".to_string()
        };
    }
}

// Define a 'Printable' trait
trait Printable {
    fn print(&self);
}

// Implement printable for square array

// Implement read_file taking a pathbuf and returning a square array
fn read_file_from_path(f: PathBuf) -> SquareArray<bool>
{
    let contents = std::fs::read_to_string(f).expect("Something went wrong reading the file");
    let mut a = SquareArray::<bool>::new();
    for (line, y) in contents.lines().zip(0..N) {
        // Remove leading and trailing '|' character from line
        let line = line.trim_matches('|');
        for (c, x) in line.chars().zip(0..N) {
            a.array[y][x] = c == 'X';
        }
    }
    a
}

fn main() {
    for i in (0..20).step_by(2) {
        println!("i is {:>2}", i);
    }

    // print current directory
    //println!("{:?}", env::current_dir());

    let resource_dir = env::current_dir().unwrap().join("resources");

    println!("Resource dir is {:?}", resource_dir);

    // define a 'squarable' trait
    trait Squarable {
        fn square(&self) -> Self;
    }

    // a macro implementing the trait for any type
    impl<T> Squarable for T
        where
            T: std::ops::Mul<Output=T> + Copy,
    {
        fn square(&self) -> Self {
            *self * *self
        }
    }

    println!("{} squared is {}", 32u32, 32u32.square());
    println!("{} squared is {}", 32u16, 32u16.square());

    let square_array = read_file_from_path(
        resource_dir.join(PathBuf::from("dat1.txt")));
    let mut aux_array = square_array.create_aux();

    #[derive(Debug)]
    struct BestResults(i32, Vec<(i32,i32)>);
    struct BestResult(i32, Vec<(i32,i32)>);
    let mut best_result: Option<BestResults> = None;

    for y in 0usize..square_array.array.len() {
        for x in 0usize..square_array.array.len() {
            if square_array.array[y][x] {
                if x == 0 || y == 0 {
                    aux_array.array[y][x] = 1;
                } else {
                    aux_array.array[y][x] = 1 + aux_array.array[y - 1][x - 1].min(aux_array.array[y - 1][x].min(aux_array.array[y][x - 1]));
                }

                // Match against best_result

                let current_val = aux_array.array[y][x];

                match &mut best_result {
                    None => {
                        best_result = Some(BestResults(aux_array.array[y][x],
                                                      vec!((y, x))));
                    }
                    Some(best) if current_val > best.0 => {
                        best.0 = aux_array.array[y][x];
                        best.1.clear();
                        best.1.push((y as i32, x as i32));
                    }
                    Some(best) if current_val == best.0 => {
                        best.1.push((y as i32, x as i32));
                    }
                    Some(_) => {}
                }
            }
        }
    }
    square_array.print();
    aux_array.print();

    let BestResult(num, results) = best_result.expect("Error");

    results.iter().for_each(|r| {
        aux_array.dunmpWithMax((r.0, r.1, num));
        println!();
        println!();
    });
    println!("Best result {:?}", best_result);
}
