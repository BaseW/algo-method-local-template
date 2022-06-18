fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();

    // trim s
    let s = s.trim();
    // print s repeat 3 times
    println!("{}", s.repeat(3));
}
