fn main() {
    // read n and v as i64 splitted by space from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut n_v: Vec<i64> = buffer
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    // initialize n and v
    let n = n_v[0];
    let v = n_v[1];

    // initialize i64 vector from stdin splitted by space
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut i64_vector: Vec<i64> = buffer
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // initialize v_found as false
    let mut v_found = false;
    // for statement for i64_vector.len() times
    for i in 0..i64_vector.len() {
        // if i64_vector[i] == v, set v_found as true
        if i64_vector[i] == v {
            v_found = true;
        }
    }

    // if v_found is true, print "Yes"
    // else, print "No"
    if v_found {
        println!("Yes");
    } else {
        println!("No");
    }
}
