fn main() {
    // read usize n and m splitted by space from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

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

    // initialize result by 0
    let mut result = 0;
    // for statement of numbers_a
    for i in 0..numbers_a.len() {
        // for statement of numbers_b
        for j in 0..numbers_b.len() {
            // if statement of numbers_a[i] > numbers_b[j]
            if numbers_a[i] + numbers_b[j] == k {
                result += 1;
            }
        }
    }

    // print result
    println!("{}", result);
}
