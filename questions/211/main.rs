fn main() {
    // read buffer from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    // parse n as i64 from buffer
    let n: i64 = buffer.trim().parse().unwrap();
    // println!("n = {}", n);

    // read buffer from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    // parse numbers splitted by space from buffer
    let numbers: Vec<i64> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    // println!("numbers: {:?}", numbers);

    let mut max_value_index = 0;
    let mut max_value = numbers[0];
    for (index, number) in numbers.iter().enumerate() {
        if *number > max_value {
            max_value = *number;
            max_value_index = index;
        }
        // println!("number = {}, max = {}", number, max_value);
    }

    println!("{}", max_value_index);
}
