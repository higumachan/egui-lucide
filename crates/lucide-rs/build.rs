use convert_case::{Case, Casing};
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;
use std::fs::read_dir;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{env, fs};

#[derive(Debug)]
struct LucideIcon {
    original_name: String,
    function_name: String,
    path_string: String,
}

fn collect_lucide_icons() -> Vec<LucideIcon> {
    read_dir("lucide/icons")
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let file_name = PathBuf::from_str(entry.file_name().to_str().unwrap()).unwrap();

            if file_name.extension().unwrap() != "svg" {
                return None;
            }

            let svg_path = fs::canonicalize(entry.path()).unwrap();
            let original_name = file_name.file_stem().unwrap().to_str().unwrap().to_string();

            let mut reader =
                Reader::from_reader(BufReader::new(std::fs::File::open(svg_path).unwrap()));
            let mut inner_buf = Vec::new();
            let mut output_buf = Vec::new();

            loop {
                match reader.read_event_into(&mut inner_buf) {
                    Ok(Event::Eof) => break,
                    Ok(Event::Start(e)) if e.name() == QName(b"svg") => {
                        // skip
                    }
                    Ok(Event::End(e)) if e.name() == QName(b"svg") => {
                        break;
                    }
                    Ok(ev) => {
                        if let Event::Empty(_) = ev {
                            output_buf.extend(b"<");
                            output_buf.extend_from_slice(&ev);
                            output_buf.extend(b"/>");
                        }
                    }
                    _ => unreachable!(),
                }
                inner_buf.clear();
            }
            let path_string = String::from_utf8(output_buf).unwrap();

            let snake_name = original_name.to_case(Case::Snake);
            Some(LucideIcon {
                original_name,
                function_name: escape_reserved(snake_name),
                path_string,
            })
        })
        .collect()
}

fn escape_reserved(name: String) -> String {
    match name.as_str() {
        "type" => "r#type".to_string(),
        "move" => "r#move".to_string(),
        "box" => "r#box".to_string(),
        _ => name,
    }
}

fn generate_icon_functions(lucide_icons: &[LucideIcon]) -> String {
    let mut icon_functions = String::new();

    for icon in lucide_icons {
        icon_functions.push_str(&format!(
            r###"/// [{0}](https://lucide.dev/icons/{0})
pub fn {1}() -> Icon {{
    Icon {{
        icon_path: r#"{2}"#,
        ..Default::default()
    }}
}}
"###,
            icon.original_name, icon.function_name, icon.path_string
        ));
    }
    icon_functions
}

fn main() {
    let lucide_icons = collect_lucide_icons();

    let out_dir = env::var("OUT_DIR").unwrap();
    let generated_file = Path::new(&out_dir).join("icon_functions.rs");
    let mut file = std::fs::File::create(generated_file).unwrap();
    let icon_enum = generate_icon_functions(&lucide_icons);
    file.write_all(icon_enum.as_bytes()).unwrap();
}
