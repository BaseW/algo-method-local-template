fn main() {
    // read a, b and c splitted by space as i64 from std input
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let mut s = s.trim().split_whitespace();
    let a: i64 = s.next().unwrap().parse().unwrap();
    let b: i64 = s.next().unwrap().parse().unwrap();
    let c: i64 = s.next().unwrap().parse().unwrap();

    // print average of a, b and c
    println!("{}", (a + b + c) / 3);
}
