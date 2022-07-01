fn main() {
    // read string s from stdin
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let s = s.trim().to_string();

    let mut dog_matched_index = 0;
    let mut cat_matched_index = 0;
    let dog_string = "dog";
    let cat_string = "cat";

    for (i, c) in s.chars().enumerate() {
        if c == dog_string.chars().nth(dog_matched_index).unwrap() {
            dog_matched_index += 1;
            cat_matched_index = 0;
        } else if c == cat_string.chars().nth(cat_matched_index).unwrap() {
            dog_matched_index = 0;
            cat_matched_index += 1;
        } else {
            dog_matched_index = 0;
            cat_matched_index = 0;
        }

        if dog_matched_index == dog_string.len() {
            break;
        } else if cat_matched_index == cat_string.len() {
            break;
        }
    }

    if dog_matched_index == dog_string.len() {
        println!("Yes");
    } else if cat_matched_index == cat_string.len() {
        println!("Yes");
    } else {
        println!("No");
    }
}
