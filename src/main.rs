use math_sheets::*;

fn main() {
    println!("{}", greeting("Yasmin"));
    println!("{}", get_operand(9, 10));
    println!("{}", create_addition_equation(2, 12));

    let addition_list = create_addition_list(25, 3, 18);
    for addition_equation in addition_list {
        println!("{}", addition_equation);
    }
}
