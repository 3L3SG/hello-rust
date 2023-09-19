const CON_X: i32 = 1;

pub fn validate(x: i32) {
    if x > CON_X {
        println!("Value {} is greater than {}", &x, &CON_X);
    } else {
        println!("Values {} is less than {}", &x, &CON_X);
    }
}

pub fn if_expression(age: i32) {
    let x = if age < 18 { "Adult" } else { "Minor" };
    println!("user is {}", &x);
}

pub fn loop_break() {
    let mut counter = 5;
    let z = loop {
        counter = counter + 1;
        println!("incremented value is {}", &counter);

        if counter == 10 { break counter * 2; }
    };
    println!("value returned is {}", &z);
}

pub fn named_loop() {
    let mut counter = 0;
    'counting_up: loop {
        println!("count = {counter}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 { break; }
            if counter == 2 { break 'counting_up; }
            remaining -= 1;
        }
        counter += 1;
    }
    println!("loop ended")
}

pub fn conditional_while(x: i32) {
    let mut counter = 1;

    while counter <= x {
        println!("While incremented value to : {counter}");
        counter += 1;
    }
}

pub fn for_loop() {
    let x = [5; 5];

    for y in x {
        println!("element is {y}");
    }
}

pub fn for_loop_generator() {
    for i in (0..5).rev() {
        println!("AutoGen element : {i}");
    }
}