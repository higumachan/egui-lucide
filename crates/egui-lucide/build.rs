use build_lucide_collect::{collect_lucide_icons, LucideIcon};
use std::env;
use std::io::Write;
use std::path::Path;

fn generate_icon_functions(lucide_icons: &[LucideIcon]) -> String {
    let mut icon_functions = String::new();

    icon_functions.push_str("impl Icon {\n");
    for icon in lucide_icons {
        icon_functions.push_str(&format!(
            r###"   /// [{0}](https://lucide.dev/icons/{0})
    pub fn {1}() -> Self {{
        Self {{ inner: lucide_rs::{1}() }}
    }}
    
"###,
            icon.original_name, icon.function_name
        ));
    }
    icon_functions.push_str("}\n");
    icon_functions
}

fn main() {
    let lucide_icons = collect_lucide_icons();

    let out_dir = env::var("OUT_DIR").unwrap();
    let generated_file = Path::new(&out_dir).join("icon_impls.rs");
    let mut file = std::fs::File::create(generated_file).unwrap();
    let icon_enum = generate_icon_functions(&lucide_icons);
    file.write_all(icon_enum.as_bytes()).unwrap();
}
