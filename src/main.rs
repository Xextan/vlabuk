use std::{fs, time::Instant};

use icu_collator::{Collator, CollatorPreferences, options::CollatorOptions};
use serde_json::Value;

fn main() {
    let start = Instant::now();
    let json = fs::read_to_string("words.json").unwrap();
    let mut wrapper = serde_json::from_str::<Value>(&json).unwrap();
    let collator =
        Collator::try_new(CollatorPreferences::default(), CollatorOptions::default()).unwrap();
    wrapper["data"].as_array_mut().unwrap().sort_by(|a, b| {
        let a_word = a["word"].as_str().unwrap_or("");
        let b_word = b["word"].as_str().unwrap_or("");
        collator.compare(a_word, b_word)
    });
    let min = serde_json::to_string(&wrapper["data"]).unwrap();
    fs::write("words.js", format!("const dict = {min};\n")).unwrap();
    let pretty = serde_json::to_string_pretty(&wrapper).unwrap() + "\n";
    fs::write("words.json", pretty).unwrap();
    println!("finished in {:?}", start.elapsed());
}
