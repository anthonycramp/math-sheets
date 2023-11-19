use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;

pub fn greeting(name: &str) -> String {
    format!("Hello, {}", name)
}

pub fn get_operand(min: i32, max: i32) -> i32 {
    let mut rng = thread_rng();
    rng.gen_range(min..=max)
}

#[derive(PartialEq)]
pub enum Operator {
    Addition,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::Addition => "+",
            }
        )
    }
}

#[derive(PartialEq)]
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
        write!(
            f,
            "{} {} {} =",
            self.first_operand, self.operator, self.second_operand
        )
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
    let mut equation_count = 1.0;
    let equation_increment = number_of_entries as f64 / (max_sum - min_sum + 1) as f64;

    for perfect_sum in min_sum..=max_sum {
        equation_count += equation_increment;
        let target_sum_between = Uniform::try_from((perfect_sum - 2)..=(perfect_sum + 2)).unwrap();
        let mut rng = rand::thread_rng();
        while equation_count > 1.0 {
            let mut target_sum = target_sum_between.sample(&mut rng);
            if target_sum < min_sum {
                target_sum = min_sum
            } else if target_sum > max_sum {
                target_sum = max_sum
            }

            let addition_equation = create_addition_equation(2, target_sum);
            if !addition_list.contains(&addition_equation) {
                addition_list.push(addition_equation);
                equation_count -= 1.0;
            }
        }
    }
    addition_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_addition_equation() {
        let min_operand = 2;
        let target_sum = 15;

        let eq = create_addition_equation(min_operand, target_sum);
        assert!(1 <= eq.first_operand && eq.first_operand <= 9);
        assert!(1 <= eq.second_operand && eq.second_operand <= 9);
        assert!(target_sum == eq.first_operand + eq.second_operand);
    }
}
