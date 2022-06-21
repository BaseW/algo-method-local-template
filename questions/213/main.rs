fn main() {
    // read buffer from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    // parse n as i64 from buffer
    let n: i64 = buffer.trim().parse().unwrap();

    // read buffer from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    // parse numbers as i64 splitted by space from buffer
    let numbers: Vec<i64> = buffer
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // initialize max_value by numbers[0]
    let mut max_value = numbers[0];
    // for statement of numbers
    for i in 1..n {
        // if statement of numbers[i]
        if numbers[i as usize] > max_value {
            // update max_value by numbers[i]
            max_value = numbers[i as usize];
        }
    }

    // print max_value
    println!("{}", max_value);
}
