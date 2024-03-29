use build_lucide_collect::{collect_lucide_icons, LucideIcon};
use convert_case::{Case, Casing};
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;
use std::fs::read_dir;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{env, fs};

fn generate_icon_functions(lucide_icons: &[LucideIcon]) -> String {
    let mut icon_functions = String::new();

    for icon in lucide_icons {
        icon_functions.push_str(&format!(
            r###"/// [{0}](https://lucide.dev/icons/{0})
pub fn {1}() -> Icon {{
    Icon {{
        icon_path: r#"{2}"#,
        name: "{0}",
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
