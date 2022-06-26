fn main() {
    // read buffer from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    // get chars from buffer
    let mut chars = buffer.trim().chars();

    // initialize result by 0
    let mut result = 0;
    // initialize previous char by chars[0]
    let mut previous_char = chars.next().unwrap();

    // iterate chars
    for char in chars {
        // if current char is same as previous char
        if char == previous_char {
            // add 1 to result
            result += 1;
        }
        // set previous char to current char
            previous_char = char;
    }

    // print result
    println!("{}", result);
}
