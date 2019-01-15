fn main() {
    another_function(5, 6);
    println!("implicit return: {}", implicit_return());
}

fn another_function(x: i32, y: i32) {
    println!("The valie of x is: {}", x);
    println!("The valie of y is: {}", y);
}

fn implicit_return() -> i32 {
    5
}
