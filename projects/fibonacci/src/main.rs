use std::io;

fn fib(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 || n == 2 {
        1
    } else {
        fib(n-1) + fib(n-2)
    }
}

fn main() {

    println!("What number:");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n: u32 = n.trim().parse().expect("Must input interger.");
    println!("n: {n}");
    println!("Result: {}", fib(n));

}
