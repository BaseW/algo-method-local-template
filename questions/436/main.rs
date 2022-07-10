fn main() {
    // read string s from stdin
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let mut s = s.trim().to_string();

    // get words from s splitted by space
    let mut words: Vec<&str> = s.split_whitespace().collect();

    let mut target_indices: Vec<usize> = Vec::new();
    let mut asian_indices: Vec<usize> = Vec::new();
    let mut word_counts_after_asian: Vec<usize> = Vec::new();

    // iterate over words
    for (i, word) in words.iter().enumerate() {
        // println!(
        //     "{}, {}, {:?}, {:?}",
        //     i, word, asian_indices, word_counts_after_asian
        // );
        if *word == "asian" {
            if asian_indices.len() > 0 {
                // all values of word_counts_after_asian increment
                for j in 0..word_counts_after_asian.len() {
                    word_counts_after_asian[j] += 1;
                }
            }
            asian_indices.push(i);
            word_counts_after_asian.push(0);
        } else if asian_indices.len() > 0 {
            // all values of word_counts_after_asian increment
            for j in 0..word_counts_after_asian.len() {
                word_counts_after_asian[j] += 1;
            }
        }
    }

    for (i, word_count_after_asian) in word_counts_after_asian.iter().enumerate() {
        if *word_count_after_asian >= 5 {
            target_indices.push(asian_indices[i]);
        }
    }

    // println!("{:?}", target_indices);

    for i in 0..words.len() {
        if target_indices.contains(&i) {
            words[i] = "global";
        }
    }

    println!("{}", words.join(" "));
}
