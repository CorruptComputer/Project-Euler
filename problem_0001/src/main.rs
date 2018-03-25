fn main() {
    let mut total = 0;
    for i in 0..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            total += i;
        }
    }

    println!("The sum of all the multiples of 3 or 5 below 1000 = {}", total);
}
