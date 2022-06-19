fn main() {
    // read s from std input
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    // use trim the newline from s as s
    s = s.trim().to_string();

    // read n and m splitted by space from std input
    let mut n_m = String::new();
    std::io::stdin().read_line(&mut n_m).ok();
    let n_m: Vec<&str> = n_m.trim().split_whitespace().collect();
    let n: i64 = n_m[0].parse().ok().unwrap();
    let m: i64 = n_m[1].parse().ok().unwrap();

    // exchange (n - 1) th char of s with (m - 1) th char of s
    let n_th = (n - 1) as usize;
    // print n_th
    // println!("{}", n_th);
    let m_th = (m - 1) as usize;
    // print m_th
    // println!("{}", m_th);
    let s_n = s.chars().nth(n_th).unwrap();
    let s_m = s.chars().nth(m_th).unwrap();
    // print s_n
    // println!("{}", s_n);
    // print s_m
    // println!("{}", s_m);
    // concatenate s
    // if index is n_th, replace with s_m
    // if index is m_th, replace with s_n
    let mut s = s.chars().collect::<String>();
    s.replace_range(n_th..=n_th, &s_m.to_string());
    s.replace_range(m_th..=m_th, &s_n.to_string());

    // print s
    println!("{}", s);
}
