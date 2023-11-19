use math_sheets::*;

fn main() {
    let addition_list = create_addition_list(25, 3, 18);
    for addition_equation in addition_list {
        println!("{}", addition_equation);
    }
}
