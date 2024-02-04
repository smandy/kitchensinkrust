use std::env;
use std::fmt::Display;
use std::path::PathBuf;

const N : usize = 9;

struct SquareArray<T : Default>  {
    array: [[T; N]; N],
}

impl<T : Default + Copy> SquareArray<T> {
    fn create_aux(&self) -> SquareArray<usize> {
        SquareArray::<usize> {
            array: [[Default::default(); N]; N],
        }
    }
    fn new() -> Self {
        SquareArray {
            // initialize array
            array: [[T::default() ; N]; N],
        }
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


// Define a 'Printable' trait
trait Printable {
    fn print(&self);
}

// Implement printable for square array
impl<T : Display + Default> Printable for SquareArray<T>
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


/*fn print_lines_from_file(f : &PathBuf) {
    let contents = std::fs::read_to_string(f).expect("Something went wrong reading the file");
    for line in contents.lines() {
        println!("{}", line);
    }
}
*/
// Implement read_file taking a pathbuf and returning a square array
fn read_file_from_path(f : PathBuf) -> SquareArray<bool>
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

/*fn read_file_from_string<const N : usize>(f : &str) -> SquareArray<bool>
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
}*/


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
        T: std::ops::Mul<Output = T> + Copy,
    {
        fn square(&self) -> Self {
            *self * *self
        }
    }

    println!("{} squared is {}", 32u32, 32u32.square());
    println!("{} squared is {}", 32u16, 32u16.square());

    let  square_array = read_file_from_path(
        resource_dir.join(PathBuf::from("dat1.txt") ));
    let mut aux_array = square_array.create_aux();

    #[derive(Debug)]
    struct BestResult {
        num : usize,
        results : Vec<(usize,usize)>,
    }

    let mut best_result : Option<BestResult>  = None;

    for y in 0..square_array.array.len() {
        for x in 0..square_array.array.len() {
            if square_array.array[y][x] {
                if x == 0 || y == 0 {
                    aux_array.array[y][x] = 1;
                } else {
                    aux_array.array[y][x] = 1 + aux_array.array[y - 1][x - 1].min(aux_array.array[y - 1][x].min(aux_array.array[y][x - 1]));
                }

                // Match against best_result
                match &mut best_result {
                    Some(best) => {
                        if aux_array.array[y][x] > best.num {
                            best.num = aux_array.array[y][x];
                            best.results.clear();
                            best.results.push((y, x));
                        } else if aux_array.array[y][x] == best.num {
                            best.results.push((y, x));
                        }
                    }
                    None => {
                        best_result = Some(BestResult {
                            num: aux_array.array[y][x],
                            results: vec![(y, x)],
                        });
                    }
                }
            }
        }
    }
    square_array.print();
    aux_array.print();

    println!("Best result {:?}", best_result);
}
