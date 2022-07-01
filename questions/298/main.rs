/// 文字列 s が標準入力から与えられる。
/// s が以下の形式を満たすか調べる。
/// metho..d (oが1つ以上)
/// 形式を満たす場合は Yes, そうでない場合は No を出力する
fn main() {
    // read string s from stdin
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let s = s.trim().to_string();

    let target_string_1 = "metho";
    let allowed_string = "o";
    let target_string_2 = "d";

    let mut matched_count_1 = 0;
    let mut matched_count_2 = 0;

    for (i, c) in s.chars().enumerate() {
        if 0 <= i && i <= 4 && matched_count_1 != target_string_1.len() {
            if c == target_string_1.chars().nth(matched_count_1).unwrap() {
                matched_count_1 += 1;
            } else {
                matched_count_1 = 0;
            }
        }
        if matched_count_1 == target_string_1.len() && matched_count_2 != target_string_2.len() {
            if c == target_string_2.chars().nth(matched_count_2).unwrap() {
                matched_count_2 += 1;
            } else if c.to_string() == allowed_string.to_string() {
                continue;
            } else {
                matched_count_1 = 0;
                matched_count_2 = 0;
            }
        }

        if matched_count_1 == target_string_1.len() && matched_count_2 == target_string_2.len() {
            break;
        }
    }

    if matched_count_1 == target_string_1.len() && matched_count_2 == target_string_2.len() {
        println!("Yes")
    } else {
        println!("No")
    }
}
