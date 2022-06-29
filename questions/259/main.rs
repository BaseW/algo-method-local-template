fn main() {
    // read n as usize from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().unwrap();

    // read n numbers from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut numbers: Vec<usize> = buffer
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // initialize result by 0
    let mut result = 0;

    for i in 0..numbers.len() {
      let x = numbers[i];
      for j in i + 1..numbers.len() {
        let y = numbers[j];
        for k in j + 1..numbers.len() {
          let z = numbers[k];
          if x <= y && y >= z {
            result += 1;
          }
        }
      }
    }

    println!("{}", result);
}
