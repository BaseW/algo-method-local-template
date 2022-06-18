fn main() {
    // read a and b splitted by space as i64 from std input
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let mut s = s.trim().split_whitespace();
    let a: i64 = s.next().unwrap().parse().unwrap();
    let b: i64 = s.next().unwrap().parse().unwrap();

    // calculate mod 10 of a as first_a
    let mut first_a = a % 10;
    // calculate mod 10 of b as first_b
    let mut first_b = b % 10;

    // print a if first_a is smaller than first_b
    if first_a < first_b {
        println!("{}", a);
    } else {
        // print b if first_a is bigger than first_b
        println!("{}", b);
    }
}
