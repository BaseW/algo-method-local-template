fn main() {
    // read buffer from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    // parse buffer as i64
    let n: i64 = buffer.trim().parse().unwrap();

    // initialize string buffer
    let mut buffer = String::new();
    // for statement n times
    for _ in 0..n {
        // read buffer from stdin
        let mut tmp_buffer = String::new();
        std::io::stdin().read_line(&mut tmp_buffer).unwrap();
        // trim tmp_buffer
        let tmp_buffer = tmp_buffer.trim();
        // concatenate s to buffer
        buffer.push_str(tmp_buffer);
    }

    // print char count of buffer
    println!("{}", buffer.chars().count());
}
