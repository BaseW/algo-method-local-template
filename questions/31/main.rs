fn main() {
    // read s from std input
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    // read t from std input
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).ok();

    // if s == t, print "Yes"
    // else, print "No"
    if s == t {
        println!("Yes");
    } else {
        println!("No");
    }
}
