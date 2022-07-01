fn main() {
    // read string s from stdin
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let s = s.trim().to_string();

    let target_str = "1+1";
    let mut matched_index = 0;

    for c in s.chars() {
        if c == target_str.chars().nth(matched_index).unwrap() {
            matched_index += 1;
        } else {
            matched_index = 0;
        }
        if target_str.len() == matched_index {
            break;
        }
    }

    if matched_index == target_str.len() {
        println!("Yes");
    } else {
        println!("No");
    }
}
