fn main() {
    // read l_r from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let l_r: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let l = l_r[0];
    let r = l_r[1];

    let mut result = 0;
    for x in l..=r {
        let x_str = x.to_string();
        let reversed_x_str = x_str.chars().rev().collect::<String>();
        if x_str == reversed_x_str {
            result += 1;
        }
    }

    println!("{}", result);
}
