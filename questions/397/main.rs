fn main() {
    // read string s from stdin
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let s = s.trim().to_string();

    // exchange "rur" to "rar" of s
    let mut exchange_target_indices = Vec::new();

    let mut first_r_found = false;
    let mut u_found = false;

    let vec_of_r = "r".chars().collect::<Vec<char>>();
    let vec_of_u = "u".chars().collect::<Vec<char>>();

    for (i, c) in s.chars().enumerate() {
        // println!("{}, {}, {}, {}", i, c, first_r_found, u_found);
        if !first_r_found {
            if vec_of_r.contains(&c) {
                first_r_found = true;
            }
        } else if first_r_found && !u_found {
            if vec_of_u.contains(&c) {
                u_found = true;
            } else if vec_of_r.contains(&c) {
                continue;
            } else {
                first_r_found = false;
            }
        } else if first_r_found && u_found {
            u_found = false;
            if vec_of_r.contains(&c) {
                exchange_target_indices.push(i - 1);
            } else {
                first_r_found = false;
            }
        }
    }

    // replace u to a in s of exchange_target_indices
    let mut s = s.clone();
    for i in exchange_target_indices {
        s.remove(i);
        s.insert(i, 'a');
    }

    println!("{}", s);
}
