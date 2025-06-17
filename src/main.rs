use notoize::{Font, NotoizeClient};
use serde_json::Value;
use std::{fs, time::Instant};

fn main() {
    let start = Instant::now();
    let words =
        serde_json::from_str::<Vec<Value>>(&fs::read_to_string("words.json").unwrap()).unwrap();
    let min = serde_json::to_string(&words).unwrap();
    fs::write("words.js", format!("const dict = {min};")).unwrap();
    // fonts (cf xlasisku)
    for font in fs::read_dir("fonts/").unwrap() {
        let font = font.unwrap();
        if let Some(name) = font.file_name().to_str() {
            if !["NotoSans-", "Iosevka-", "nokiapiqad"]
                .iter()
                .any(|x| name.starts_with(x))
            {
                fs::remove_file(font.path()).unwrap();
            }
        }
    }
    let mut client = NotoizeClient::new();
    let mut fonts = client.notoize(min.as_str()).files();
    fonts.retain(|f| f.fontname != "Noto Sans");
    fonts.push(Font {
        bytes: fs::read("fonts/nokiapiqad.ttf").unwrap(),
        filename: "nokiapiqad.ttf".to_string(),
        fontname: "Nokia Pure HL KLGN".to_string(),
    });
    let mut css = String::new();
    for font in fonts.clone() {
        fs::write(format!("fonts/{}", font.filename), font.bytes).unwrap();
        css = format!(
            "{css}@font-face {{\r\n    font-family: \"{}\";\r\n    src: url(\"fonts/{}\");\r\n    \
             font-display: swap;\r\n}}\r\n",
            font.fontname, font.filename
        );
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
    println!(" in {:?}", start.elapsed());
}
