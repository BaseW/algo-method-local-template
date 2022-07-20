fn main() {
    // read buffer from string
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    // parse buffer as usize n
    let n: usize = buffer.trim().parse().unwrap();

    // read buffer from string
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    // parse buffer as array of usizes
    let mut array: Vec<usize> = buffer
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for i in 1..n {
        let mut pos = i;
        while pos != 0 && array[pos - 1] > array[pos] {
            let temp = array[pos - 1];
            array[pos - 1] = array[pos];
            array[pos] = temp;
            pos -= 1;
        }
        for i in 0..n {
            print!("{}", array[i]);
            if i != n - 1 {
                print!(" ");
            }
        }
        println!("");
    }
}
