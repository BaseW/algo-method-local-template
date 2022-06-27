/// count factor of n
fn count_factor(n: usize) -> usize {
    let mut count = 0;
    let mut i = 1;
    for i in 1..=n {
        if n % i == 0 {
            count += 1;
        }
    }
    count
}

fn main() {
    // read n_k from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n_k: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = n_k[0];
    let k = n_k[1];

    let mut result = 0;
    for i in 1..=n {
        if count_factor(i) == k {
            result += 1;
        }
    }

    println!("{}", result);
}
