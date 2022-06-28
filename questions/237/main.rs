fn is_kaibun(s: &str) -> bool {
    let reversed_s = s.chars().rev().collect::<String>();
    s == &reversed_s
}

fn main() {
    // read usize n from stdin
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    // initialize Vec<String>
    let mut s = Vec::new();

    // read s_n from stdin n times
    for _ in 0..n {
        let mut s_n = String::new();
        std::io::stdin().read_line(&mut s_n).unwrap();
        s.push(s_n.trim().to_string());
    }

    // initialize kaibun_count by 0
    let mut kaibun_count = 0;
    // check s_n is kaibun or not
    for s_n in s.iter() {
        // if s_n is kaibun, increment kaibun_count
        if is_kaibun(s_n) {
            kaibun_count += 1;
        }
    }

    // print kaibun_count
    println!("{}", kaibun_count);
}
