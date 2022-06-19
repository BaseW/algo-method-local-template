fn main() {
    // read s from std input
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    // use trim the newline from s as s
    s = s.trim().to_string();

    // read t from std input
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).ok();
    // use trim the newline from t as t
    t = t.trim().to_string();

    // read u from std input
    let mut u = String::new();
    std::io::stdin().read_line(&mut u).ok();
    // use trim the newline from u as u
    u = u.trim().to_string();

    // print concatenation of u, t and s
    println!("{}{}{}", u, t, s);
}
