fn main() {

  iteration();

  println!("{:?}", convert_temp(100.0, 'f'));
  println!("{:?}", convert_temp(100.0, 'c'));
  println!("{:?}", convert_temp(100.0, 'x'));

  println!("{:?}", generate_n_fibonacci(10));

  println!("{:?}", fizzbuzz_up_to(15));
}


fn fancy_fizzbuzz_up_to(n: u32) -> Vec<String> {
  (1..n).map(|i| {
    match (i % 3 == 0, i % 5 == 0) {
      (true, true) => "fizzbuzz".to_string(),
      (true, false) => "fizz".to_string(),
      (false, true) => "buzz".to_string(),
      (false, false) => i.to_string(),
    }
  }).collect()
}

fn fizzbuzz_up_to(n: u32) -> Vec<String> {
  let mut result = Vec::with_capacity(n as usize);
  for mut i in 1..=n {
    if i % 3 == 0 && i % 5 == 0 {
      result.push("fizzbuzz".to_string());
    } else if i % 3 == 0 {
      result.push("fizz".to_string());
    } else if i % 5 == 0 {
      result.push("buzz".to_string());
    } else {
      result.push(i.to_string());
    }
  }
  result
}

fn generate_n_fibonacci(n: usize) -> Vec<i32> {
  let mut result = Vec::with_capacity(n);
  if n == 0 {
    return result;
  }
  
  result.push(0);
  result.push(1);

  for i in 2..n {
    result.push(result[i - 1] + result[i - 2]);
  }

  result
}

fn convert_temp(temp: f64, unit: char) -> Result<f64, String> {
  match unit {
    'c' => Ok(convert_to_fahrenheit(temp)),
    'f' => Ok(convert_to_celsius(temp)),
    _ => Err(format!("Invalid unit: {}", unit)),
  }
}

fn convert_to_celsius(fahrenheit: f64) -> f64 {
  (fahrenheit - 32.0) * (5.0 / 9.0)
}

fn convert_to_fahrenheit(celsius: f64) -> f64 {
  (celsius * (9.0 / 5.0)) + 32.0
}

fn iteration() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    };
    println!("The result is {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
    let mut index = 0;
    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    for number in (1..4).rev() {
        println!("{number}!");
    }
}

