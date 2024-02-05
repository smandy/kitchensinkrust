use std::env;
use std::fmt::Display;
//use std::fmt::Display;
use std::path::PathBuf;

const N: usize = 9;

#[derive(Clone)]
struct SquareArray<T: Default + Repr> {
    array: [[T; N]; N],
}

#[derive(Debug)]
struct BestResults(u8, Vec<(usize, usize)>);

#[derive(Debug)]
struct BestResult(u8, usize, usize);


impl<T: Default + Copy + Repr> SquareArray<T> {
    fn create_aux(&self) -> SquareArray<u8> {
        SquareArray::<u8> {
            array: [[Default::default(); N]; N],
        }
    }

    fn print(&self, best: Option<&BestResult>) {
        for i in 0..N {
            for j in 0..N {
                match best {
                    Some(BestResult(_n, _y, _x)) => {
                        let (n, y, x) = (*_n as usize, *_y, *_x);
                        let y_range_start = if y >= n { y - n + 1 } else { 0 };
                        let x_range_start = if x >= n { x - n + 1 } else { 0 };

                        print!("{}", if (y_range_start..=y).contains(&i) && (x_range_start..=x).contains(&j)
                            {"X "} else {". "});
                    }
                    _ => {
                        print!("{} ", self.array[i][j].repr());
                    }
                }
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

impl Repr for u8 {
    fn repr(&self) -> String {
        return format!("{}", self);
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
    let resource_dir = env::current_dir().unwrap().join("resources");

    println!("Resource dir is {:?}", resource_dir);


    let square_array = read_file_from_path(
        resource_dir.join(PathBuf::from("dat1.txt")));
    let mut aux_array = square_array.create_aux();

    let none = None;
    let mut best_result: Option<BestResults> = none;

    for y in 0usize..square_array.array.len() {
        for x in 0usize..square_array.array.len() {
            if square_array.array[y][x] {
                if x == 0 || y == 0 {
                    aux_array.array[y][x] = 1;
                } else {
                    aux_array.array[y][x] = 1 + aux_array.array[y - 1][x - 1].min(aux_array.array[y - 1][x].min(aux_array.array[y][x - 1]));
                }

                let current_val = aux_array.array[y][x];

                match &mut best_result {
                    None => {
                        best_result = Some(BestResults(aux_array.array[y][x],
                                                       vec!((y, x))));
                    }
                    Some(ref mut best) if current_val > best.0 => {
                        best.0 = aux_array.array[y][x];
                        best.1.clear();
                        best.1.push((y, x));
                    }
                    Some(ref mut best) if current_val == best.0 => {
                        best.1.push((y, x));
                    }
                    Some(_) => {}
                }
            }
        }
    }
    square_array.print(None);
    println!();
    aux_array.print(None);
    println!();

    let BestResults(num, results) = best_result.as_ref().expect("Error");

    results.iter().for_each(|(y, x)| {
        let br = BestResult(*num, *y, *x);
        println!("====================================");
        aux_array.print(Some(&br));
        println!();
    });

    impl Display for BestResults {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Best result is {} at {:?}", self.0, self.1)
        }
    }

    //let s = best_result.as_ref().map(|s| s.to_string());

    println!("Best result {:?}", best_result);
}

