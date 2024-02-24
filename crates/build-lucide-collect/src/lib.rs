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
pub struct LucideIcon {
    pub original_name: String,
    pub function_name: String,
    pub path_string: String,
}

pub fn collect_lucide_icons() -> Vec<LucideIcon> {
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
