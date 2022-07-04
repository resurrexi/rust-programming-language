fn main() {
    another_function(5, 'h');

    let x = five();
    println!("The value of x is: {}", x);

    let y = plus_one(5);
    println!("The value of y is: {}", y);
}

fn another_function(x: i32, unit_label: char) {
    println!("The measurement is: {}{}", x, unit_label);
}

fn five() -> i32 {
    // note there's no semicolon to imply a returned value
    5
}

fn plus_one(x: i32) -> i32 {
    // note there's no semicolon to imply a returned value
    x + 1
}
