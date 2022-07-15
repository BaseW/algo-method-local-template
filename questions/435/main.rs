fn main() {
    // read string s from stdin
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let s = s.trim().to_string();

    let necessary_chars = "algo".chars();
    let unnecessary_chars_1 = "rithm".chars();
    let unnecessary_chars_2 = "method".chars();
    let mut necessary_chars_indices: Vec<usize> = Vec::new();
    let mut is_after_necessary_chars_ok_list: Vec<bool> = Vec::new();

    for i in 0..s.len() {
        // println!(
        //     "{}, {:?}, {:?}",
        //     i, necessary_chars_indices, is_after_necessary_chars_ok_list
        // );
        let necessary_chars_cloned = necessary_chars.clone();
        let unnecessary_chars_1_cloned = unnecessary_chars_1.clone();
        let unnecessary_chars_2_cloned = unnecessary_chars_2.clone();

        if s.len() <= i + 4 {
            continue;
        }
        // get chars from s of length 4
        let first_four_chars = s[i..i + 4].chars();

        // check if first_four_chars is "algo"
        if !first_four_chars.eq(necessary_chars_cloned) {
            continue;
        }

        // if it is, add the index to necessary_chars_indices
        // this index may be ok
        necessary_chars_indices.push(i);

        let mut is_after_necessary_chars_ok = true;

        if s.len() < i + 9 {
            is_after_necessary_chars_ok_list.push(false);
            continue;
        }
        // get chars from s after first_four_chars of length 5
        let after_first_four_chars_1 = s[i + 4..i + 9].chars();
        // println!("{:?}", after_first_four_chars_1);
        if after_first_four_chars_1.eq(unnecessary_chars_1_cloned) {
            // not ok
            is_after_necessary_chars_ok_list.push(false);
            continue;
        }

        if s.len() < i + 10 {
            is_after_necessary_chars_ok_list.push(is_after_necessary_chars_ok);
            continue;
        }
        let after_first_four_chars_2 = s[i + 4..i + 10].chars();
        // println!("{:?}", after_first_four_chars_2);
        if after_first_four_chars_2.eq(unnecessary_chars_2_cloned) {
            // not ok
            is_after_necessary_chars_ok_list.push(false);
            continue;
        }

        is_after_necessary_chars_ok_list.push(is_after_necessary_chars_ok);
    }

    let mut result = false;
    for i in 0..is_after_necessary_chars_ok_list.len() {
        if is_after_necessary_chars_ok_list[i] == true {
            result = true;
            break;
        }
    }

    if result {
        println!("Yes");
    } else {
        println!("No");
    }
}
