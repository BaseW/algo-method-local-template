fn main() {
    // read s from std input
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();

    // trim s
    let s = s.trim();

    // parse s as u8
    let s: u8 = s.parse().unwrap();

    // print 24 - s
    println!("{}", 24 - s);
}
