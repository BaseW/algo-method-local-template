fn main() {
    // read string s from stdin
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let s = s.trim().to_string();

    let mut is_s_matched = false;
    let mut number_matched_count = 0;
    let mut hyphen_found = false;
    let number_chars = "0123456789".chars().collect::<Vec<_>>();
    let hyphen_chars = "-".chars().collect::<Vec<_>>();

    for c in s.chars() {
        // println!("{}, {}", c, number_matched_count);
        if number_matched_count < 3 {
            if number_chars.contains(&c) {
                number_matched_count += 1;
            } else {
                number_matched_count = 0;
            }
        } else if number_matched_count == 3 {
            if !hyphen_found {
                if !hyphen_chars.contains(&c) {
                    number_matched_count = 0;
                } else {
                    hyphen_found = true;
                }
            } else {
                if number_chars.contains(&c) {
                    number_matched_count += 1;
                } else {
                    number_matched_count = 0;
                }
            }
        } else {
            if number_chars.contains(&c) {
                number_matched_count += 1;
            } else {
                number_matched_count = 0;
            }
            if number_matched_count == 7 {
                is_s_matched = true;
            }
        }
    }

    if is_s_matched {
        println!("Yes");
    } else {
        println!("No");
    }
}
