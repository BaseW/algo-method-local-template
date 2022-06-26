/// check if number is prime number
fn is_prime(n: u8) -> bool {
    if n == 1 {
        return false;
    }
    if n == 2 || n == 3 || n == 5 || n == 7 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn main() {
    // read u8 from stdin
    let mut u8_str = String::new();
    std::io::stdin().read_line(&mut u8_str).unwrap();
    // parse as u8 n
    let n: u8 = u8_str.trim().parse().unwrap();

    // read numbers splitted by space from stdin
    let mut numbers_str = String::new();
    std::io::stdin().read_line(&mut numbers_str).unwrap();
    // parse as u8 numbers
    let numbers: Vec<u8> = numbers_str
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // initialize result by 0
    let mut result = 0;

    // iterate over numbers
    for number in numbers {
        // if number is prime, add to result
        if is_prime(number) {
            result += 1;
        }
    }

    // print result
    println!("{}", result);
}
