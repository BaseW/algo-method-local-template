fn main() {
    // read string s from stdin
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let s = s.trim().to_string();

    let mut is_s_matched = false;
    let mut matched_count = 0;
    let number_chars = "0123456789".chars().collect::<Vec<_>>();

    for c in s.chars() {
        if number_chars.contains(&c) {
            matched_count += 1;
        } else {
            matched_count = 0;
        }
        if matched_count == 3 {
            is_s_matched = true;
        }
    }

    if is_s_matched {
        println!("Yes");
    } else {
        println!("No");
    }
}
