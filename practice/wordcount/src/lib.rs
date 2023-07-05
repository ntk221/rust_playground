use regex::Regex;
use std::collections::HashMap;
use std::io::{BufRead};

// pubにすることで，ライブラリの外から参照することができる
pub fn count(input: impl BufRead) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();
        // 単語で分割する
        for m in re.find_iter(&line) {
            // 単語の出現回数をカウントする
            let word = m.as_str().to_string();
            *freqs.entry(word).or_insert(0) += 1;
        }   
    }
    freqs
}