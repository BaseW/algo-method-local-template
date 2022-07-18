fn is_valid(target_year: String, target_month: String, name: String) -> (bool, String) {
    // name format is {number}_{receipt_name}_YYYYMMDD.pdf
    // get number, receipt_name, and date
    let mut parts = name.split('_');
    let _number = parts.next().unwrap();
    let receipt_name = parts.next().unwrap();
    let date = parts.next().unwrap();
    // get year and month from "YYYYMM"
    let year = &date[..4];
    let month = &date[4..6];
    let target_month = if target_month.len() == 1 {
        format!("0{}", target_month)
    } else {
        target_month.clone()
    };
    // check if year and month match target
    if year != target_year || month != target_month {
        return (false, "".to_string());
    }
    (true, receipt_name.to_string())
}

fn main() {
    // read string buffer from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    // get n, y and m from buffer splitted by space
    let buffer_splitted: Vec<&str> = buffer.split_whitespace().collect();
    let n = buffer_splitted[0].parse::<i32>().unwrap();
    let y = buffer_splitted[1];
    let m = buffer_splitted[2];

    // initialize names by Vec<string>
    let mut names: Vec<String> = Vec::new();
    // read string, then push names
    for _ in 0..n {
        let mut name = String::new();
        std::io::stdin().read_line(&mut name).unwrap();
        names.push(name.trim().to_string());
    }

    // initialize result by Vec<string>
    let mut result: Vec<String> = Vec::new();

    // check names, if valid, push to result
    for name in names {
        let (is_valid_name, receipt_name) = is_valid(
            y.clone().to_string(),
            m.clone().to_string(),
            name.as_str().to_string(),
        );
        if is_valid_name {
            result.push(receipt_name);
        }
    }

    // print names in result
    for name in result {
        println!("{}", name);
    }
}
