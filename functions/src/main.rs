use std::cmp::Ordering;

fn main() {
    fn nesting(){
        println!("nested function");
        add(24,44);
    }
    println!("Hello, world!");
    new_func();
    nesting();
    let fact: i32 = return_factorial(8);
    println!("Factorial: {fact}");
}

fn new_func() {
    println!("New function");
}

fn add(x: i32, y:i32){
    let sum = x + y;
    println!("Sum: {sum}");
}

fn return_factorial(mut x: i32) -> i32 {
    let mut prod = 1;
    const _ZERO: i32 = 0;
    loop {
        match x.cmp(&_ZERO) {
            Ordering::Less => continue,
            Ordering::Greater => {
                prod = prod * x;
                x = x - 1;
            },
            Ordering::Equal => break,
        }
    }
    prod
}
