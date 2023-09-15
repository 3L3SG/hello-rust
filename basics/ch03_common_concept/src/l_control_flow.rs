const CON_X: i32 = 1;

pub fn valdiate(x: i32) {
    if x > CON_X {
        println!("Value {} is greate than {}", &x, &CON_X);
    } else {
        println!("Values {} is less than {}", &x, &CON_X);
    }
}
