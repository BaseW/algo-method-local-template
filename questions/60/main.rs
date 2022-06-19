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

    // calculate average of numbers as f32
    let average: f32 = numbers.iter().sum::<i64>() as f32 / numbers.len() as f32;
    // round average
    let average = average.floor();
    // print average
    println!("{}", average);
}
