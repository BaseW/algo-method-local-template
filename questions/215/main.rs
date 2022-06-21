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

    // initialize larger_than_before_count by 0
    let mut larger_than_before_count = 0;
    // for statement of numbers
    for i in 1..n {
        // if statement of numbers[i]
        if numbers[i as usize] > numbers[(i - 1) as usize] {
            // increment larger_than_before_count by 1
            larger_than_before_count += 1;
        }
    }

    // print larger_than_before_count
    println!("{}", larger_than_before_count);
}
