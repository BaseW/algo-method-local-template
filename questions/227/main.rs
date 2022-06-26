fn check_if_string_is_kaibun(s: String) -> bool {
    s.chars().rev().collect::<String>() == s
}

fn main() {
    // read buffer from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    if check_if_string_is_kaibun(buffer.trim().to_string()) {
        println!("Yes");
    } else {
        println!("No");
    }
}
