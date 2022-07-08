fn main() {
    // read string s from stdin
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let s = s.trim().to_string();

    let small_prints = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect::<Vec<_>>();
    let at_mark = "@".chars().collect::<Vec<_>>();
    let dot = ".".chars().collect::<Vec<_>>();
    let mut matched_small_print_count = 0;
    let mut at_mark_found = false;
    let mut dot_found = false;

    for (i, c) in s.chars().enumerate() {
        // check if s is match with following conditions:
        // (1 or more small prints)@(1 or more small prints).(more than 1 and less than 4 small prints)
        // if at_mark_found is false, c must be small print.
        // if dot_found is false, c must be small print.
        // println!(
        //     "{}, {}, {}, {}",
        //     c, matched_small_print_count, at_mark_found, dot_found
        // );
        if !at_mark_found {
            if small_prints.contains(&c) {
                matched_small_print_count += 1;
            } else if matched_small_print_count > 0 && at_mark.contains(&c) {
                matched_small_print_count = 0;
                at_mark_found = true;
            } else {
                break;
            }
        } else if at_mark_found && !dot_found {
            if small_prints.contains(&c) {
                matched_small_print_count += 1;
            } else if matched_small_print_count > 0 && dot.contains(&c) {
                matched_small_print_count = 0;
                dot_found = true;
            } else {
                break;
            }
        } else {
            if small_prints.contains(&c) {
                matched_small_print_count += 1;
            } else {
                break;
            }
        }
    }

    if at_mark_found
        && dot_found
        && 1 <= matched_small_print_count
        && matched_small_print_count <= 4
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
