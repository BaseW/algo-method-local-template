/// read inputs from stdin
/// splitted by newline
/// as u8, String, String
fn read_inputs() -> Vec<String> {
    let mut inputs = Vec::new();
    for i in 0..3 {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        inputs.push(buffer.trim().to_string());
    }
    inputs
}

/// compare two Strings, then return count of different chars
fn compare_strings(a: &str, b: &str) -> usize {
    let mut count = 0;
    for (i, c) in a.chars().enumerate() {
        if c != b.chars().nth(i).unwrap() {
            count += 1;
        }
    }
    count
}

fn main() {
    // read inputs
    let inputs = read_inputs();
    // parse first element of inputs as u8
    let n = inputs[0].parse::<u8>().unwrap();
    // parse second element of inputs as String s
    let s = inputs[1].to_string();
    // parse third element of inputs as String t
    let t = inputs[2].to_string();

    // compare s and t
    let count = compare_strings(&s, &t);

    // print result
    println!("{}", count);
}
