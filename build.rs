use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct FontFile {
    characters: BTreeMap<char, Vec<Vec<f32>>>,
}

struct Font {
    characters: BTreeMap<char, Vec<f32>>,
}

impl From<FontFile> for Font {
    fn from(font_file: FontFile) -> Self {
        let mut characters = BTreeMap::new();
        for (c, v) in font_file.characters {
            let mut vertices = Vec::new();
            for v in v {
                if v.len() < 4 {
                    continue;
                }

                for i in 0..(v.len() - 2) / 2 {
                    let index = i * 2;
                    vertices.push(v[index]);
                    vertices.push(v[index + 1]);
                    vertices.push(v[index + 2]);
                    vertices.push(v[index + 3]);
                }
            }

            characters.insert(c, vertices);
        }
        Self { characters }
    }
}

fn quote_char(c: char) -> String {
    match c {
        '\n' => "'\\n'".to_string(),
        '\r' => "'\\r'".to_string(),
        '\t' => "'\\t'".to_string(),
        '\\' => "'\\\\'".to_string(),
        '\'' => "'\\''".to_string(),
        _ => format!("'{}'", c),
    }
}

impl Font {
    fn write_to_rs_file<P: AsRef<Path>>(&self, rs_file: P) {
        use std::io::Write;
        let file = std::fs::File::create(rs_file).unwrap();
        let mut file = std::io::BufWriter::new(file);

        writeln!(
            file,
            "pub const CHARACTER_VERTICES : [(char, &'static [f32]);{}] = [",
            self.characters.len()
        )
        .unwrap();

        for (c, v) in &self.characters {
            write!(file, "    ({}, &[", quote_char(*c)).unwrap();
            for e in v {
                if *e == (*e as i32 as f32) {
                    write!(file, "{}.0,", e).unwrap();
                } else {
                    write!(file, "{},", e).unwrap();
                }
            }
            writeln!(file, "]),").unwrap();
        }

        writeln!(file, "];").unwrap();
    }
}

fn main() {
    let font_file = std::fs::read_to_string("system-text-font.json").unwrap();
    let font_file: FontFile = serde_json::from_str(&font_file).unwrap();

    let font: Font = font_file.into();

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let rs_file = std::path::Path::new(&out_dir).join("system_text_font.rs");
    font.write_to_rs_file(rs_file);
    println!("cargo:rerun-if-changed=font.json");
}
