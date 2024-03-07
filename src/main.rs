use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Entry {
    word: String,
    concept: String, // meaning
    class: String,
    status: String,
    source: Source,
    definition: String, // alignment / particle fn
    semantics: String,
    notes: String,
    gloss: String,
}
struct Source {
    language: String,
    word: String,
}

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
    // parse the words
    // cf Xextan/parser
    let root_str = &format!(
        "^({}|{})$",
        "[pbtdkgfvszxqln][aeiou][ptkln]", // cvf
        "([sx][ptk]|[zq][bdg]|[szxq][ln]|[pbkgfv]l|t[sx]|d[zq])[aeiou]"  // ccv
    );
    let root = Regex::new(root_str).unwrap();
    let words: Vec<_> = words
        .iter()
        .map(|word| {
            let head: Vec<_> = word.split_whitespace().collect();
            if root.is_match(head[0]) {
                println!("root: {}", head[0]);
            }
        })
        .collect();
}
