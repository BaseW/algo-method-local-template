fn main() {
    // read buffer from stdin and parse i64 value n
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n: i64 = buffer.trim().parse().unwrap();

    // read buffer from stdin and parse numbers as i64 splitted by space
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let numbers: Vec<i64> = buffer
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // initialize over_0_count by 0
    let mut over_0_count = 0;
    // for statement of numbers
    for i in 0..n {
        // if statement of numbers[i]
        if numbers[i as usize] > 0 {
            // increment over_0_count by 1
            over_0_count += 1;
        }
    }

    // print over_0_count
    println!("{}", over_0_count);
}
