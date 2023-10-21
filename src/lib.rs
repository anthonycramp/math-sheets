use rand::prelude::*;

pub fn greeting(name: &str) -> String {
    format!("Hello, {}", name)
}

pub fn get_operand(min: i32, max: i32) -> i32 {
    let mut rng = thread_rng();
    rng.gen_range(min..=max)
}

pub fn get_addition_equation(min_operand: i32, target_sum: i32) -> String {
    let mut rng = thread_rng();
    let upper_bound = std::cmp::min(10, target_sum);
    let mut first_operand = rng.gen_range(min_operand..upper_bound);
    while target_sum - first_operand >= 10 {
        first_operand = rng.gen_range(min_operand..upper_bound);
    }
    let second_operand = target_sum - first_operand;

    format!("{} + {} =", first_operand, second_operand)
}

pub fn create_addition_list(number_of_entries: i32, min_sum: i32, max_sum: i32) -> Vec<String> {
    let mut addition_list = vec![]; 
    for target_sum in min_sum..=max_sum {
        addition_list.push(get_addition_equation(2, target_sum));
    }
    addition_list
}
