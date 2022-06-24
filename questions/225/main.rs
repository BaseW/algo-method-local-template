fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n: u8 = buffer.trim().parse().unwrap();

    for i in 1..=n {
        let mod_3 = i % 3;
        let mod_5 = i % 5;
        if mod_3 == 0 && mod_5 == 0 {
          println!("FizzBuzz");
        } else if mod_3 == 0 {
          println!("Fizz");
        } else if mod_5 == 0 {
          println!("Buzz");
        } else {
          println!("{}", i);
        }
    }
}
