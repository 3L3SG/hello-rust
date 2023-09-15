pub fn variables() {
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

pub fn var_shadowing() {
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