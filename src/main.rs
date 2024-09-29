fn fib(n: u32) -> u32 {
    if n < 2 {
        // The base case.
        if n == 1 {1} else {0}
    } else {
        // The recursive case.
        fib(n-1)+fib(n-2)
    }
}

fn main() {
    let n = 20;
    let i = 0;
    for i in i..=n {
        println!("fib({i}) = {}", fib(i));
    }
}