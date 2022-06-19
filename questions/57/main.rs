fn main() {
    // read buffer from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    // parse buffer as i64
    let n: i64 = buffer.trim().parse().unwrap();

    // read n numbers splitted by space from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    // parse buffer as Vec<i64>
    let numbers: Vec<i64> = buffer
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // get reversed_numbers
    let mut reversed_numbers = numbers.clone();
    reversed_numbers.reverse();

    // repeat printing the number of reversed_numbers
    for i in 0..n {
        println!("{}", reversed_numbers[i as usize]);
    }
}
