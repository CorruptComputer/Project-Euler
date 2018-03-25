fn main() {

    let mut prev = 1;
    let mut fib = 2;
    let mut even_total = 0;

    while fib < 4000000 {
        if fib % 2 == 0 {
            even_total += fib;
        }

        let temp = fib;
        fib += prev;
        prev = temp;
    }

    println!("Total of even Fibonacci numbers less than 4,000,000: {}", even_total);
}
