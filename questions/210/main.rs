fn main() {
    // read buffer from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    // parse n and v as i64 splitted by space from buffer
    let nv: Vec<i64> = buffer
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = nv[0];
    let v = nv[1];

    // read n numbers from stdin splitted by space
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut numbers: Vec<i64> = buffer
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // initialize i64 value v_count by 0
    let mut v_count = 0;
    // for statement of numbers
    for i in 0..n {
        // if statement of numbers[i]
        if numbers[i as usize] == v {
            // increment v_count by 1
            v_count += 1;
        }
    }

    // print v_count
    println!("{}", v_count);
}
