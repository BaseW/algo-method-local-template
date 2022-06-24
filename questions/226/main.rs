fn read_string_from_stdin() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

fn main() {
    let s = read_string_from_stdin();
    let c = read_string_from_stdin();

    let mut is_c_included = false;
    let chars_of_s = s.chars();
    for char_of_s in chars_of_s {
        if char_of_s.to_string() == c {
            is_c_included = true;
        }
    }

    if is_c_included {
        println!("Yes");
    } else {
        println!("No");
    }
}
