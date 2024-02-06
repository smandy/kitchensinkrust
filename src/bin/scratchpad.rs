fn main() {
    println!("Wayhey");

    let sz = 4;
    let corner = 8;

    for i in corner-sz+1..=corner {
        println!("i is {:>2}", i);
    }

    let x = vec!((2,3), (4,5));

    println!("{:?}", x);

    let xs : Vec<i32> = (0..10).collect();

    println!("{:?}", xs);
}
