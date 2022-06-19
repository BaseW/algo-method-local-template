fn main() {
    // read s as string from std input
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    // use trim the newline from s as s
    s = s.trim().to_string();

    // read i64 value from std input as t
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).ok();
    // parse t as i64
    t = t.trim().to_string();
    let t: i64 = t.parse().ok().unwrap();

    // print (t - 1) th char of s
    println!("{}", s.chars().nth((t - 1) as usize).unwrap());
}
