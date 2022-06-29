fn main() {
    // read n and k as usize splitted by space from stdin
    let mut nk = String::new();
    std::io::stdin().read_line(&mut nk).unwrap();
    let mut iter = nk.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    // read numbers splitted by space from stdin
    let mut numbers = String::new();
    std::io::stdin().read_line(&mut numbers).unwrap();
    let numbers: Vec<usize> = numbers
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // initialize result by 0
    let mut result = 0;
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            if i == j {
                continue;
            }
            let element_i = numbers[i];
            let element_j = numbers[j];
            if element_i + element_j <= k {
                result += 1
            }
        }
    }

    // print result
    println!("{}", result);
}
