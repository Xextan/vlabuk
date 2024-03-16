use minify::json::minify_from_read;
use notoize::NotoizeClient;
use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    let mut min = String::new();
    let _ = minify_from_read(File::open("words.json").unwrap()).read_to_string(&mut min);
    fs::write("words.js", format!("const dict = {min};")).unwrap();
    // fonts (cf xlasisku)
    let client = NotoizeClient::new();
    let mut fonts = client.clone().notoize(min.as_str()).files();
    fonts.retain(|f| !["Noto Sans"].contains(&f.fontname.as_str()));
    drop(client);
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
