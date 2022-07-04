fn main() {
    // read string s from stdin
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let s = s.trim().to_string();

    let english_small_prints = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>();
    let hyphen = "-".chars().collect::<Vec<_>>();

    let mut does_s_matched = false;
    let mut hyphen_found = false;
    let chars_of_s = s.chars();
    let chars_of_s_count = s.chars().count();

    for (i, char_of_s) in chars_of_s.enumerate() {
        // println!("{}, {}, {}", i, char_of_s, does_s_matched);
        if i == 0 || i == chars_of_s_count - 1 {
            if hyphen.contains(&char_of_s) {
                does_s_matched = false;
                break;
            } else if english_small_prints.contains(&char_of_s) {
                does_s_matched = true;
            } else {
                does_s_matched = false;
                break;
            }
        } else {
            if !english_small_prints.contains(&char_of_s) && !hyphen.contains(&char_of_s) {
                does_s_matched = false;
                break;
            } else {
                if !hyphen_found {
                    does_s_matched = true;
                    if hyphen.contains(&char_of_s) {
                        hyphen_found = true;
                    } else {
                        hyphen_found = false;
                    }
                } else {
                    if hyphen.contains(&char_of_s) {
                        does_s_matched = false;
                        break;
                    } else {
                        hyphen_found = false;
                    }
                }
            }
        }
    }

    if does_s_matched {
        println!("Yes");
    } else {
        println!("No");
    }
}
