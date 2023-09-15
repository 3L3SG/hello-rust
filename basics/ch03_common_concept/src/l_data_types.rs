use std::io;

pub fn data_types() {
    let x: u8 = 24;
    println!("{x}");

    let x: f32 = 21.23;
    println!("{x}");

    let x: bool = true;
    println!("{x}");
}

pub fn arrays_tuples() {
    let x = (1, 2.0, 4);
    let y = x.0;
    println!("{y}");

    let x = [22, 23, 24, 25];
    let y = x[0];
    println!("{y}");

    let x = [5; 3];
    let y = x[0];
    println!("{y}");
}

pub fn test_index() {
    let x = [10, 20, 30, 40, 50];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read the line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Oops, index entered was not a number!!");

    let element = x[index];
    println!("{element}")
}
