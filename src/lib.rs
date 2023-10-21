use rand::prelude::*;

pub fn greeting(name: &str) -> String {
    format!("Hello, {}", name)
}

pub fn get_operand(min: i32, max: i32) -> i32 {
    let mut rng = thread_rng();
    rng.gen_range(min..=max)
}
