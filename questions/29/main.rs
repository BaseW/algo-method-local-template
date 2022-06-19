fn main() {
    // read a, b, c and d splitted by space as i64 from std input
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let mut s = s.trim().split_whitespace();
    let a: i64 = s.next().unwrap().parse().unwrap();
    let b: i64 = s.next().unwrap().parse().unwrap();
    let c: i64 = s.next().unwrap().parse().unwrap();
    let d: i64 = s.next().unwrap().parse().unwrap();

    // print max of a, b, c and d
    println!(
        "{}",
        std::cmp::max(a, std::cmp::max(b, std::cmp::max(c, d)))
    );
}
