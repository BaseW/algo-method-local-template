fn main() {
    // read buffer from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer);
    // parse i64 as n from buffer
    // let n: i64 = buffer.trim().parse().unwrap();

    // read buffer from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer);
    // parse numbers as Vec<i64> from buffer
    let numbers: Vec<i64> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut value_counts: Vec<usize> = Vec::new();
    for _ in 0..9 {
        value_counts.push(0);
    }

    for number in numbers {
        let index = (number - 1) as usize;
        let current_value = value_counts[index] as usize;
        value_counts[index] = current_value + 1;
    }

    let mut max_value_index: usize = 0;
    let mut max_value_count = value_counts[0];
    for (index, value_count) in value_counts.iter().enumerate() {
        if *value_count > max_value_count {
            max_value_count = *value_count;
            max_value_index = index;
        }
    }

    println!("{}", max_value_index + 1);
}
