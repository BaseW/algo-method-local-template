fn main() {
    // read buffer from stdin and parse i64 value n and v splitted by space
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let nv: Vec<i64> = buffer
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = nv[0];
    let v = nv[1];

    // read buffer from stdin and parse numbers as i64 splitted by space
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let numbers: Vec<i64> = buffer
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // initialize v_found_index by -1
    let mut v_found_index = -1;

    // for statement of numbers
    for i in 0..numbers.len() {
        // if statement of numbers[i]
        if numbers[i as usize] == v {
            // set v_found_index by i
            v_found_index = i as i64;
        }
    }

    // print v_found_index
    println!("{}", v_found_index);
}
