use std::{fs, time::Instant};

use serde_json::Value;

fn main() {
    let start = Instant::now();
    let json = fs::read_to_string("words.json").unwrap();
    let wrapper = serde_json::from_str::<Value>(&json).unwrap();
    let words = wrapper["data"].as_array().unwrap();
    let min = serde_json::to_string(&words).unwrap();
    fs::write("words.js", format!("const dict = {min};")).unwrap();
    println!("finished in {:?}", start.elapsed());
}
