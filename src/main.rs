// use serde::{Deserialize, Serialize};
use notoize::NotoizeClient;
use serde_json::Value;
use std::fs;

fn main() {
    let min = serde_json::to_string(
        &serde_json::from_str::<Vec<Value>>(&fs::read_to_string("words.json").unwrap()).unwrap(),
    )
    .unwrap();
    fs::write("words.js", format!("const dict = {min}.sort(function(a, b) {{return a.word.localeCompare(b.word);}});")).unwrap();
    // fonts (cf xlasisku)
    let client = NotoizeClient::new();
    let mut fonts = client.clone().notoize(min.as_str()).files();
    fonts.retain(|f| !["Noto Sans"].contains(&f.fontname.as_str()));
    let mut css = String::new();
    for font in fonts.clone() {
        fs::write(format!("fonts/{}", font.filename), font.bytes).unwrap();
        css = format!("{css}@font-face {{\r\n    font-family: \"{}\";\r\n    src: url(\"fonts/{}\");\r\n    font-display: swap;\r\n}}\r\n", font.fontname, font.filename);
    }
    css = format!(
        "{css}:root {{\r\n    --sans: \"Noto Sans\", {}, ui-sans-serif, sans-serif;\r\n}}",
        fonts
            .iter()
            .map(|f| format!("\"{}\"", f.fontname))
            .collect::<Vec<_>>()
            .join(", ")
    );
    fs::write("noto.css", css).unwrap();
}
