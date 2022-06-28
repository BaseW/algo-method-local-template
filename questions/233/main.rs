fn main() {
    // read usize n and m splitted by space from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let x: usize = iter.next().unwrap().parse().unwrap();
    let y: usize = iter.next().unwrap().parse().unwrap();
    let z: usize = iter.next().unwrap().parse().unwrap();

    // read Vec<usize> numbers_a splitted by space from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let numbers_a: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // read Vec<usize> numbers_b splitted by space from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let numbers_b: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // read Vec<usize> numbers_b splitted by space from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let numbers_c: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // initialize result by 0
    let mut result = 0;
    // i is 0 <= n - 1, j is 0 <= m - 1
    // numbers_a.len() is n
    // numbers_b.len() is m
    // check count for numbers_a[i] > numbers_b[j]
    for i in 0..numbers_a.len() {
        for j in 0..numbers_b.len() {
            for k in 0..numbers_c.len() {
                if numbers_a[i] + numbers_b[j] == numbers_c[k] {
                    result += 1;
                }
            }
        }
    }

    // print result
    println!("{}", result);
}
