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
    println!("Recived value from function : {x}");
    println!();

    l_control_flow::valdiate(5);
    println!();

}
