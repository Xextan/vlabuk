use std::{
    fs::{self, File},
    io::Write as _,
    time::Instant,
};

use icu_collator::{Collator, CollatorPreferences, options::CollatorOptions};
use regex::Regex;
use serde::{Deserialize, Serialize};
use unicode_normalization::UnicodeNormalization as _;

#[derive(Debug, Serialize, Deserialize)]
struct Dictionary {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub data: Vec<Word>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Word {
    #[allow(clippy::struct_field_names, reason = "it's the word")]
    word: String,
    def: String,
    #[serde(skip_serializing_if = "is_empty_or_none")]
    derivs: Option<Derivs>,
    #[serde(skip_serializing_if = "is_empty_or_none")]
    gloss: Option<String>,
    #[serde(skip_serializing_if = "is_empty_or_none", rename = "type")]
    typ: Option<String>,
    #[serde(skip_serializing_if = "is_empty_or_none")]
    alignment: Option<String>,
    #[serde(skip_serializing_if = "is_empty_or_none")]
    semantics: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    etymology: Option<Etymology>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notes: Option<Notes>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum Notes {
    Text(String),
    EvilHtml {
        #[serde(rename = "EVIL_DANGEROUS_HTML")]
        evil_dangerous_html: String,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(deny_unknown_fields)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    zi: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum Etymology {
    Text(String),
    Single(EtymologyEntry),
    Multiple(Vec<EtymologyItem>),
}
#[expect(clippy::ref_option, reason = "needed for serde")]
fn is_empty_or_none<T: Default + PartialEq>(t: &Option<T>) -> bool {
    t.as_ref().is_none_or(|v| *v == T::default())
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum EtymologyItem {
    Entry(EtymologyEntry),
    Text(String),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct EtymologyEntry {
    lang: String,
    word: String,
    #[serde(skip_serializing_if = "is_empty_or_none")]
    translit: Option<String>,
    #[serde(skip_serializing_if = "is_empty_or_none")]
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
    for word in &mut dict.data {
        word.word = word.word.nfc().collect();
    }
    let collator =
        Collator::try_new(CollatorPreferences::default(), CollatorOptions::default()).unwrap();
    dict.data.sort_by(|a, b| collator.compare(&a.word, &b.word));
    let root = Regex::new(
        "^([bdfgklnpqstvxz][aeiou][klnpt]|([zq][bdgln]|[sx][ptkln]|[pbkgfv]l|d[zq]|t[sx])[aeiou])$",
    )
    .unwrap();
    let mut roots_tsv = File::create("roots.tsv").unwrap();
    for word in &dict.data {
        if root.is_match(&word.word) {
            writeln!(roots_tsv, "{}\t{}", word.word, word.def).unwrap();
        }
    }
    let pretty = serde_json::to_string_pretty(&dict).unwrap() + "\n";
    fs::write("words.json", pretty).unwrap();
    println!("finished in {:?}", start.elapsed());
}
