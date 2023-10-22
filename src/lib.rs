use rand::prelude::*;
use std::fmt;

pub fn greeting(name: &str) -> String {
    format!("Hello, {}", name)
}

pub fn get_operand(min: i32, max: i32) -> i32 {
    let mut rng = thread_rng();
    rng.gen_range(min..=max)
}

pub enum Operator {
    Addition,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Operator::Addition => "+",
        })
    }
}

pub struct Equation {
    pub first_operand: i32,
    pub second_operand: i32,
    pub operator: Operator,
}

impl Equation {
    fn new(first_operand: i32, second_operand: i32, operator: Operator) -> Self {
        Self {
            first_operand,
            second_operand,
            operator,
        }
    }
}

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} =", self.first_operand, self.operator, self.second_operand)
    }
}

pub fn create_addition_equation(min_operand: i32, target_sum: i32) -> Equation {
    let mut rng = thread_rng();
    let upper_bound = std::cmp::min(10, target_sum);
    let mut first_operand = rng.gen_range(min_operand..upper_bound);
    while target_sum - first_operand >= 10 {
        first_operand = rng.gen_range(min_operand..upper_bound);
    }
    let second_operand = target_sum - first_operand;

    Equation::new(first_operand, second_operand, Operator::Addition)
}

pub fn create_addition_list(number_of_entries: i32, min_sum: i32, max_sum: i32) -> Vec<Equation> {
    let mut addition_list = vec![]; 
    for target_sum in min_sum..=max_sum {
        addition_list.push(create_addition_equation(2, target_sum));
    }
    addition_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_addition_equation() {

    }
}