fn square(x: i32) -> i32 {
    x * x  // no semicolon: this is the return value
}

fn check_number(x: i32) -> &'static str {
    if x < 0 {
        return "Negative number!";
    }
    "Number is positive or zero."
}

pub fn functions() {
    let num = -5;
    println!("Square of {} is {}", num, square(num));
    println!("{}", check_number(num));

    let num2 = 10;
    println!("Square of {} is {}", num2, square(num2));
    println!("{}", check_number(num2));
}
