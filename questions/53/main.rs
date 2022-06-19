fn main() {
    // read buffer from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let buffer = buffer.trim();
    // parse buffer as i64
    let n: i64 = buffer.parse().unwrap();

    // read n numbers splitted by space from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let buffer = buffer.trim();
    let numbers: Vec<i64> = buffer
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // calculate sum of numbers
    let sum: i64 = numbers.iter().sum();
    // print sum
    println!("{}", sum);
}
