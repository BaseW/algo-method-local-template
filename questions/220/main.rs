fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer);
    let n: usize = buffer.trim().parse().unwrap();

    let mut target_count = 0;
    for i in 1..=n {
        let mod_2 = i % 2;
        let mod_3 = i % 3;
        let mod_5 = i % 5;

        if mod_2 != 0 && mod_3 != 0 && mod_5 != 0 {
            target_count += 1;
        }
    }

    println!("{}", target_count);
}
