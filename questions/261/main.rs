fn main() {
    // read string from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    // parse string as usize n
    let n: usize = buffer.trim().parse().unwrap();

    // read string as s from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    // parse string as s
    let s: String = buffer.trim().to_string();

    // initialize result by 0
    let mut result = 0;

    for i in 0..n {
        let x = s.chars().nth(i).unwrap();
        for j in i + 1..n {
            let y = s.chars().nth(j).unwrap();
            if x == y {
                result += 1;
            }
        }
    }

    println!("{}", result);
}
