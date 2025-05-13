use std::collections::HashMap;

pub struct NumberData {
    numbers: Vec<i32>,
}

impl NumberData {
    pub fn new(numbers: Vec<i32>) -> Self {
        Self { numbers }
    }
    
    pub fn median(&self) -> f64 {
        let mut numbers = self.numbers.clone();
        numbers.sort();

        let mid = numbers.len() / 2;

        if numbers.len() % 2 == 0 {
            (numbers[mid - 1] + numbers[mid]) as f64 / 2.0
        } else {
            numbers[mid] as f64
        }
    }

    pub fn mode(&self) -> i32 {
        let mut counter = HashMap::new();
        for &number in &self.numbers {
            let count = counter.entry(number).or_insert(0);
            *count += 1;
        }

        counter.into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(number, _)| number)
            .unwrap()
    }
}
