fn main() {
    // read a and b splitted by space as i64 from std input
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let mut s = s.trim().split_whitespace();
    let a: i64 = s.next().unwrap().parse().unwrap();
    let b: i64 = s.next().unwrap().parse().unwrap();

    // judge that a is a multiple of b
    // if a is a multiple of b, print Yes
    // if a is not a multiple of b, print No
    if a % b == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
