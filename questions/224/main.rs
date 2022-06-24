fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer);
    let a_b: Vec<u8> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let a = a_b[0];
    let b = a_b[1];

    // GCD: Greatest Common Divisor
    let mut gcd = 1;
    let min_value = a.min(b);

    if min_value != 1 {
        for i in 2..=min_value {
            if a % i == 0 && b % i == 0 {
                gcd = i;
            }
        }
    }
    println!("{}", gcd);
}
