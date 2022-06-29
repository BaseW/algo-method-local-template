fn main() {
    // read buffer from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().unwrap();

    // initialize Vec<String>
    let mut string_array: Vec<String> = Vec::new();

    // read n strings from stdin
    for _ in 0..n {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        string_array.push(buffer.trim().to_string());
    }

    let mut same_string_found = false;
    for i in 0..string_array.len() {
        for j in i + 1..string_array.len() {
            if string_array[i] == string_array[j] {
                same_string_found = true;
                break;
            }
        }
    }

    if same_string_found {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}
