fn main() {
    // read s from std input
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();

    // trim s
    let s = s.trim();

    // print third char of s
    println!("{}", s.chars().nth(2).unwrap());
}
