fn main() {
    // read buffer from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer);
    // parse i64 as n from buffer
    let n: i64 = buffer.trim().parse().unwrap();

    // read buffer from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer);
    // parse numbers as Vec<i64> from buffer
    let numbers: Vec<i64> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut min_value = numbers[0];
    for number in numbers {
      if number < min_value {
        min_value = number;
      }
    }

    println!("{}", min_value);
}
