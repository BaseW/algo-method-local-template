fn read_n_and_array() -> (usize, Vec<usize>) {
    // read buffer from stdin
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    // parse buffer as usize n
    let n: usize = buffer.trim().parse().unwrap();

    // read buffer from stdin
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    // parse buffer as array of usizes
    let array: Vec<usize> = buffer
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (n, array)
}

fn sort_array_by_quick_sort(array: Vec<usize>) -> Vec<usize> {
    let n = array.len();

    if n == 0 {
        let empty_vec: Vec<usize> = Vec::new();
        return empty_vec;
    }

    let x = (n as f32 / 2 as f32).floor() as usize;
    let mut l: Vec<usize> = Vec::new();
    let mut r: Vec<usize> = Vec::new();
    for i in 0..n {
        if i == x {
            continue;
        }
        if array[i] < array[x] {
            l.push(array[i]);
        } else {
            r.push(array[i]);
        }
    }
    let mut sorted_r = sort_array_by_quick_sort(r);
    let mut sorted_l = sort_array_by_quick_sort(l);
    // println!("sorted_l: {:?}", sorted_l);
    // println!("sorted_r: {:?}", sorted_r);
    let mut result: Vec<usize> = Vec::new();
    result.append(&mut sorted_l);
    result.push(array[x]);
    result.append(&mut sorted_r);
    result
}

fn print_array(array: Vec<usize>) {
    let n = array.len();
    for i in 0..n {
        print!("{}", array[i]);
        if i != n - 1 {
            print!(" ");
        }
    }
    println!("");
}

fn main() {
    let (_, array) = read_n_and_array();
    let sorted_array = sort_array_by_quick_sort(array);
    print_array(sorted_array);
}
