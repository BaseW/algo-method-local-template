fn main() {
    // read buffer from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    // parse buffer as i64
    let n: i64 = buffer.trim().parse().unwrap();

    // initialize string buffer
    let mut buffer = String::new();
    // initialize left count by 0
    let mut left_count = 0;
    // initialize right count by 0
    let mut right_count = 0;
    // for statement n times
    for _ in 0..n {
        // read buffer from stdin
        let mut tmp_buffer = String::new();
        std::io::stdin().read_line(&mut tmp_buffer).unwrap();
        // trim tmp_buffer
        let tmp_buffer = tmp_buffer.trim();
        // if tmp_buffer == "left", increment left count
        if tmp_buffer == "left" {
            left_count += 1;
        } else {
            // else, increment right count
            right_count += 1;
        }
    }

    // if left_count > right_count, print "left"
    // else if right_count > left_count, print "right"
    // else, print "same"
    if left_count > right_count {
        println!("left");
    } else if right_count > left_count {
        println!("right");
    } else {
        println!("same");
    }
}
