fn main() {
    // read string s from stdin
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let s = s.trim().to_string();

    // check if s meets following conditions:
    //   s starts with "a...a", and 1 <= count_a <= 5
    //   following string of s is "b...b", and count_b = 10
    //   following string of s is "c...c", and count_c  >= 0
    let mut does_s_meets_conditions = false;

    let mut count_a = 0;
    let mut count_b = 0;
    for (i, c) in s.chars().enumerate() {
        // println!("{}, {}, {}, {}", i, c, count_a, count_b);
        if 0 <= i && i <= 4 {
            if c.to_string() == "a".to_string() {
                count_a += 1
            } else if c.to_string() == "b".to_string() && 1 <= count_a && count_a <= 5 {
                count_b += 1;
            } else {
                does_s_meets_conditions = false;
                break;
            }
        } else {
            if c.to_string() == "b".to_string() {
                count_b += 1
            } else if c.to_string() == "c".to_string() {
                continue;
            } else {
                does_s_meets_conditions = false;
                break;
            }
        }
        if count_a == 0 && count_a > 5 {
            does_s_meets_conditions = false;
            break;
        }
    }

    if 0 <= count_a && count_a <= 5 && count_b == 10 {
        does_s_meets_conditions = true;
    }

    if does_s_meets_conditions {
        println!("Yes");
    } else {
        println!("No");
    }
}
