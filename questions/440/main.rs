fn main() {
    // read buffer from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    // parse buffer as usize n
    let n: usize = buffer.trim().parse().unwrap();
    // read buffer from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    // parse buffer as array of usizes
    let mut array: Vec<usize> = buffer
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for i in 0..n - 1 {
        let mut min_value = array[i];
        let mut min_value_index = i;
        for j in i + 1..n {
            // println!("{}, {}", j, array[j]);
            if min_value > array[j] {
                min_value = array[j];
                min_value_index = j;
            }
        }
        // println!("{}, {}, {}", i, min_value, min_value_index);
        let temp = array[min_value_index];
        array[min_value_index] = array[i];
        array[i] = min_value;

        for i in 0..n {
            print!("{}", array[i]);
            if i != n - 1 {
                print!(" ");
            }
        }
        println!("");
    }
}
