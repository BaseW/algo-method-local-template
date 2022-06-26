/// returns whether s includes t in sequence
///
/// #Arguments
/// * s - string to search in
/// * t - string to search for
///
/// #Returns
/// * true if s includes t, false otherwise
fn includes(s: &str, t: &str) -> bool {
    let mut i = 0;
    let mut j = 0;
    while i < s.len() && j < t.len() {
        if s.chars().nth(i).unwrap() == t.chars().nth(j).unwrap() {
            i += 1;
            j += 1;
        } else {
            i += 1;
            j = 0;
        }
        if j == t.len() {
            break;
        }
    }
    j == t.len()
}

fn main() {
    // read s from stdin
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    // read t from stdin
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    // trim s
    s.pop();
    // trim t
    t.pop();

    // check if s includes t
    let result = includes(&s, &t);

    // if result is true, print Yes, otherwise print No
    if result {
        println!("Yes");
    } else {
        println!("No");
    }
}
