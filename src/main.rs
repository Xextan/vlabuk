use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let data = File::open("words.txt").expect("where data");
    let lines = BufReader::new(data).lines();
    let mut in_comment = false;
    let mut words: Vec<String> = vec![];
    for line in lines {
        let line = line.unwrap();
        if !in_comment && line == "/*" {
            in_comment = true;
        }
        let skip = line.is_empty() || line.starts_with("//") || in_comment;
        if !skip {
            // println!("{line}");
            if line.starts_with("---") {
                words.push(String::new());
            } else {
                words.last_mut().unwrap().push_str(&format!("\r\n{line}"));
            }
        }
        if in_comment && line == "*/" {
            in_comment = false;
        }
    }
    words.retain(|word| !word.is_empty());
    // strip \r\n from start
    words = words.iter().map(|word| word[2..].to_string()).collect();
    // println!("{words:?}");
    todo!();
}
