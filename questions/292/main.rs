fn main() {
    // read string s from stdin
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let s = s.trim().to_string();

    let target_string = "algo";
    let mut matched_count = 0;

    for (_, c) in s.chars().enumerate() {
        if c == target_string.chars().nth(matched_count).unwrap() {
            matched_count += 1;
        } else {
            matched_count = 0;
        }
        if matched_count == target_string.len() {
            // println!("{}", i - target_string.len() + 1);
            break;
        }
    }

    if matched_count == target_string.len() {
        println!("Yes");
    } else {
        println!("No");
    }
}
