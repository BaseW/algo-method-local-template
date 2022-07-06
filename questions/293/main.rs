fn main() {
    // read string s from stdin
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let s = s.trim().to_string();

    let mut does_s_include_number = false;
    let number_chars = "0123456789".chars().collect::<Vec<_>>();

    for c in s.chars() {
        if number_chars.contains(&c) {
            does_s_include_number = true;
            break;
        }
    }

    if does_s_include_number {
        println!("Yes");
    } else {
        println!("No");
    }
}
