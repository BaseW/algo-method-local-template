fn main() {
    // read n as i64 from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n: i64 = buffer.trim().parse().unwrap();

    // read n numbers splitted by space from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let numbers: Vec<i64> = buffer
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // initialize result by 1
    let mut result = 1;
    // iterate for numbers
    for number in numbers {
        // calculate result
        result *= number;
    }
    // print result
    println!("{}", result);
}
