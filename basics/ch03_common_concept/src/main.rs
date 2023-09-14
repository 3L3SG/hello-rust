use std::io;

const RAND_NUM: u32 = 23 - 21;

fn variables() {
    let x = 10;
    println!("Initial value : {}", &x);

    //immutables caanot reassigned
    //x=15;
    println!("After reassignment : {}", &x);

    let mut y = 20;
    println!("Initial value : {}", &y);

    y = 25;
    println!("After reassignment : {}", &y);
}

fn var_shadowing() {
    let x = 10;
    let x = x + 5;
    {
        let x = x * 2;
        println!("value OF inner shadowed 'x' {}", &x);
    }

    println!("value OF outer shadowed 'x' {}", &x);

    let x = "Damn, shadowing concept is awesome!!";
    println!("{x}");
}

fn data_types() {
    let x: u8 = 24;
    println!("{x}");

    let x: f32 = 21.23;
    println!("{x}");

    let x: bool = true;
    println!("{x}");
}

fn arrays_tuples() {
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

fn test_index() {
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

fn main() {
    variables();
    println!();

    println!("And the constant used value is {}", 26 - RAND_NUM);
    println!();

    var_shadowing();
    println!();

    data_types();
    println!();

    arrays_tuples();
    println!();

    test_index();
    println!()
}
