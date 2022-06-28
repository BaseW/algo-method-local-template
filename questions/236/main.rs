use std::collections::HashMap;

fn main() {
    // read s from stdin
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    let chars_of_s = s.chars();
    // get array of english small printable characters
    let small_printable_chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    // get map of english small printable characters to found
    let mut small_printable_chars_map: HashMap<char, bool> = HashMap::new();
    for c in small_printable_chars.iter() {
        small_printable_chars_map.insert(*c, false);
    }

    // if char of s is in small printable chars, set found to true
    for c in chars_of_s {
        if small_printable_chars_map.contains_key(&c) {
            small_printable_chars_map.insert(c, true);
        }
    }

    // get count of found chars
    let mut count = 0;
    for (_, found) in small_printable_chars_map.iter() {
        if *found {
            count += 1;
        }
    }

    // print count
    println!("{}", count);
}
