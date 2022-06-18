fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Please provide number");
    // println!("buffer: {}", buffer);

    let number: u8 = buffer.trim().parse().expect("parse buffer error");
    // println!("number = {}", number);

    let doubled_number = number * 2;
    println!("{}", doubled_number);
}
