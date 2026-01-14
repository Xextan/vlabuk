use std::{fs, time::Instant};

use icu_collator::{Collator, CollatorPreferences, options::CollatorOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Dictionary {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub data: Vec<Word>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::struct_field_names)]
struct Word {
    word: String,
    def: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    derivs: Option<Derivs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gloss: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    word_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alignment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    semantics: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    etymology: Option<Etymology>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notes: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Derivs {
    #[serde(skip_serializing_if = "Option::is_none")]
    xo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ko: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ga: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    qu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    po: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    se: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    si: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zu: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum Etymology {
    Single(EtymologyEntry),
    Multiple(Vec<EtymologyItem>),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum EtymologyItem {
    Entry(EtymologyEntry),
    Connector(String), // "+" or "←"
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct EtymologyEntry {
    lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    word: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    translit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    urlform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<Link>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum Link {
    Bool(bool),
    Url(String),
}

fn main() {
    let start = Instant::now();
    let json = fs::read_to_string("words.json").unwrap();
    let mut dict = serde_json::from_str::<Dictionary>(&json).unwrap();
    let collator =
        Collator::try_new(CollatorPreferences::default(), CollatorOptions::default()).unwrap();
    dict.data.sort_by(|a, b| collator.compare(&a.word, &b.word));
    let min = serde_json::to_string(&dict.data).unwrap();
    fs::write("words.js", format!("const dict = {min};\n")).unwrap();
    let pretty = serde_json::to_string_pretty(&dict).unwrap() + "\n";
    fs::write("words.json", pretty).unwrap();
    println!("finished in {:?}", start.elapsed());
}
