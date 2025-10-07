const CONSTANT: u32 = 42;

fn main() {
    println!("The answer is {CONSTANT}");
    let spaces = "    ";
    let mut spaces = spaces.len();
    spaces = spaces.wrapping_pow(2);
    println!("{spaces}");
    println!("{}", -5. / 3.);
    println!("{}", -5 / 3); // rounds towards zero

    let t = true;
    let f: bool = false;
    let c: char = '😻'; // char is 4 bytes in rust, encoding Unicode values
    println!("{t}{f}{c}");

    compound_types();
    function_with_params(5);
}

fn compound_types() {
    /* Compound Types */
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (_, y, _) = tuple; // destructure via pattern matching
    let other_y = tuple.1;

    println!("{y} and {other_y}");

    let array: [i32; 4] = [1, 2, 3, 4];
    let array_b = [3; 5]; // [3, 3, 3, 3, 3]
    println!("{}, {}", array[0], array_b[4]); // 1, 3
    println!("{}", five());
}

fn function_with_params(x: i32) {
    println!("{x}");
}

/* 
* - *Statements* are instructions that perform some action and do not return a value
* - *Expressions* evaluate to a resultant value
*/

fn five() -> i32 {
    5
}
