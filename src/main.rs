use std::{fs, time::Instant};

use icu_collator::{Collator, CollatorPreferences, options::CollatorOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dictionary {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub data: Vec<Word>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Word {
    pub word: String,
    pub def: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derivs: Option<Derivs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gloss: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub word_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantics: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etymology: Option<Etymology>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Derivs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ko: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ga: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub po: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub si: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Etymology {
    Single(EtymologyEntry),
    Multiple(Vec<EtymologyItem>),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EtymologyItem {
    Entry(EtymologyEntry),
    Connector(String), // "+" or "←"
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtymologyEntry {
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urlform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Link {
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
