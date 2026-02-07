use std::io::stdin;

fn main() {
    let numbers = loop_1();
    println!("Even numbers is: {}", loop_2(&numbers));
    println!("Sum numbers is: {}", loop_3(&numbers));
}

fn loop_1() -> Vec<i32> {
    let mut numbers = Vec::new();
    loop {
        println!("Enter a number or 0 to exite:");
        let mut i_number_str = String::new();
        stdin()
            .read_line(&mut i_number_str)
            .expect("Ther is an error");
        let i_number: i32 = match i_number_str.trim().parse() {
            Ok(o) => o,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        if i_number == 0 {
            break;
        } else {
            numbers.push(i_number);
        }
    }
    numbers
}

fn loop_2(numbers: &Vec<i32>) -> i32 {
    let mut even = 0;
    for n in numbers {
        if n % 2 == 0 {
            even += 1;
        }
    }
    even
}

fn loop_3(numbers: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for n in numbers {
        sum += n;
    }
    sum
}
