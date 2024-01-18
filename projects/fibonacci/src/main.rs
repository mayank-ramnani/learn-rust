use std::io;
use std::io::Write;

fn fib(n: u32) -> u32 {
    if n == 1 || n == 2 {
       return 1; 
    }
    return fib(n-1) + fib(n-2)
}
// 5: f(4) + f(3) = 5
// 4: f(3) + f(2) = 2 + 1 = 3
// 3: f(2) + f(1) = 1 + 1 = 2
// 1, 1, 2, 3, 5, 8, 13, 21, 34

fn main() {
    // user input n
    println!("Nth Fibonacci Number");
    print!("Enter n: ");
    io::stdout().flush().unwrap();
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n: u32 = match n.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Bad Input");
            return;
        }
    };
    if n == 0 {
        println!("Bad Input");
        return;
    }
    println!("The {n}th fibonacci number is: {}", fib(n));
}
