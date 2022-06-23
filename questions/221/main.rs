fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n: u8 = buffer.trim().parse().unwrap();

    let mut factor_count: u8 = 0;
    for i in 1..=n {
        if n % i == 0 {
            factor_count += 1;
        }
    }

    println!("{:?}", factor_count);
}
