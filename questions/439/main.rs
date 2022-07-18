fn main() {
    // read buffer from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    // parse usize n from buffer
    let n: usize = buffer.trim().parse().unwrap();

    // read buffer from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    // parse as array of usizes
    let mut array: Vec<usize> = buffer
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut is_exchange_executed = true;
    while is_exchange_executed {
        is_exchange_executed = false;
        for i in 0..n - 1 {
            if array[i] > array[i + 1] {
                let temp = array[i];
                array[i] = array[i + 1];
                array[i + 1] = temp;
                is_exchange_executed = true;
            }
        }
        if !is_exchange_executed {
            break;
        }
        let mut array_string = String::new();
        for i in 0..n {
            array_string.push_str(&array[i].to_string());
            if i != n - 1 {
                array_string.push_str(" ");
            }
        }
        println!("{}", array_string);
    }
}
