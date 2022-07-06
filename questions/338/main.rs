fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let mut s = s.trim().to_string();

    let mut is_bracket_opened = false;
    let mut is_internal_found = false;
    let mut is_brackets_found = false;

    for c in s.chars() {
        // println!(
        //     "{}, {}, {}, {}",
        //     c, is_bracket_opened, is_internal_found, is_brackets_found
        // );
        if c == '(' {
            if is_bracket_opened {
                is_internal_found = true;
            } else {
                is_bracket_opened = true;
            }
        } else if c == ')' {
            if is_bracket_opened {
                if is_internal_found {
                    is_brackets_found = true;
                    break;
                }
                is_bracket_opened = false;
                is_internal_found = true;
            }
        } else {
            if is_bracket_opened {
                is_internal_found = true;
            }
        }
    }

    if is_brackets_found {
        println!("Yes");
    } else {
        println!("No");
    }
}
