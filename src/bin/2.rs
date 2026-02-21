use std::io::stdin;
fn main() {
    let mut my_array: [i32; 3] = [0; 3];

    loop {
        println!("Enter 0 or 1:");
        let mut i = String::new();
        stdin().read_line(&mut i).expect("Error");
        let n: i32 = match i.trim().parse() {
            Ok(n) => n,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        if n == 0 {
            loop_1(&mut my_array);
        } else if n == 1 {
            loop_2(&mut my_array);
        } else {
            println!("The number is not 0 or 1");
        }
    }
}

fn loop_1(my_array: &mut [i32; 3]) {
    let mut n: usize = my_array.len();
    while n >= my_array.len() {
        println!("Input index:");
        let mut i = String::new();
        stdin().read_line(&mut i).expect("Error");
        n = match i.trim().parse() {
            Ok(n) => n,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
    }

    println!("The value of index is: {}", my_array[n]);
}

fn loop_2(my_array: &mut [i32; 3]) {
    let mut n: usize = my_array.len();
    let mut value = 0;
    while n >= my_array.len() {
        println!("Input index:");
        let mut i = String::new();
        stdin().read_line(&mut i).expect("Error");
        n = match i.trim().parse() {
            Ok(n) => n,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        println!("Input value:");
        let mut i = String::new();
        stdin().read_line(&mut i).expect("Error");
        value = match i.trim().parse() {
            Ok(n) => n,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
    }

    my_array[n] = value;
    println!("The value of index:{} updated to: {}", n, value);
}
