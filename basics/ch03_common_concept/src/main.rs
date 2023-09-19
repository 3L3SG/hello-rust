use crate::l_control_flow::{conditional_while, for_loop, for_loop_generator, if_expression, loop_break, named_loop};

mod l_data_types;
mod l_functions;
mod l_vars_mut;
mod l_control_flow;

const RAND_NUM: u32 = 23 - 21;

fn main() {
    l_vars_mut::variables();
    println!();

    println!("And the constant used value is {}", 26 - RAND_NUM);
    println!();

    l_vars_mut::var_shadowing();
    println!();

    l_data_types::data_types();
    println!();

    l_data_types::arrays_tuples();
    println!();

    l_data_types::test_index();
    println!();

    l_functions::another_function(24, 'M');
    println!();

    let x = l_functions::ret_five_plus(2);
    println!("Received value from function : {x}");
    println!();

    l_control_flow::validate(5);
    println!();

    if_expression(20);
    println!();

    loop_break();
    println!();

    named_loop();
    println!();

    conditional_while(5);
    println!();

    for_loop();
    println!();

    for_loop_generator();
    println!();
}
