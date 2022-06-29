fn main() {
    // read l and r as usize splitted by space from stdin
    let mut lr = String::new();
    std::io::stdin().read_line(&mut lr).unwrap();
    let mut iter = lr.split_whitespace();
    let l: usize = iter.next().unwrap().parse().unwrap();
    let r: usize = iter.next().unwrap().parse().unwrap();

    // initialize result by 0
    let mut result = 0;

    // loop from l to r
    for i in l..r + 1 {
        // loop from i to r
        for j in i + 1..r + 1 {
            let first_place_of_i = i % 10;
            let first_place_of_j = j % 10;
            if first_place_of_i == first_place_of_j {
                result += 1;
            }
        }
    }

    // print result
    println!("{}", result);
}
