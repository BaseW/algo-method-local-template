fn main() {
    // read string from std input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // parse input as i32
    let input = input.trim().parse::<i32>().unwrap();

    // calculate input mod 5
    let input = input % 5;
    // print result
    println!("{}", input);
}
