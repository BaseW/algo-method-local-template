fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer);
    let n: u8 = buffer.trim().parse().unwrap();

    let mut is_factor_found = false;
    if n == 1 {
        is_factor_found = true;
    } else {
        for i in 2..n {
            if n % i == 0 {
                is_factor_found = true;
                break;
            }
        }
    }

    if is_factor_found {
        println!("No");
    } else {
        println!("Yes");
    }
}
