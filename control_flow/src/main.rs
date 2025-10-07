use std::io;

fn main() {
    let number = read_number();

    if number < 5 {
        println!("true");
    } else {
        println!("false");
    }
    multiple_loops();

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("a[{index}] = {}", a[index]);

        index += 1;
    }

    // For loops are not only safer but can also be more efficient
    for (index, element) in a.iter().enumerate() {
        println!("a[{index}] = {element}");
    }

    for num in (1..4).rev() {
        println!("{num}");
    }
}

fn read_number() -> i32 {
    let mut number = String::new();

    println!("Please input a number!");

    loop {
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read input");

        let number: i32 = match number.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };
        return number;
    }
}

fn multiple_loops() {
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
    }
    println!("endcount = {count}");
}
