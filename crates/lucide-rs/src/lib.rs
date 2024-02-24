pub struct Icon {
    icon_path: &'static str,
    size: u32,
    color: Color,
    stroke_width: u32,
}

impl Default for Icon {
    fn default() -> Self {
        Icon {
            icon_path: "",
            size: 24,
            color: Color {
                red: 0,
                green: 0,
                blue: 0,
            },
            stroke_width: 2,
        }
    }
}

impl Icon {
    pub fn svg(&self) -> String {
        let mut content = String::new();
        content.push_str(&format!(
            "<svg width=\"{0}\" height=\"{0}\" viewBox=\"0 0 {0} {0}\" fill=\"none\" stroke = \"rgb({1}, {2}, {3})\" stroke-linecap=\"round\" stroke-linejoin=\"round\"  xmlns=\"http://www.w3.org/2000/svg\">",
            self.size,
            self.color.red,
            self.color.green,
            self.color.blue
        ));
        content.push_str(self.icon_path);
        content.push_str("</svg>");
        content
    }

    pub fn size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn stroke_width(mut self, stroke_width: u32) -> Self {
        self.stroke_width = stroke_width;
        self
    }
}

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn red() -> Self {
        Color {
            red: 255,
            green: 0,
            blue: 0,
        }
    }

    pub fn green() -> Self {
        Color {
            red: 0,
            green: 255,
            blue: 0,
        }
    }

    pub fn blue() -> Self {
        Color {
            red: 0,
            green: 0,
            blue: 255,
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/icon_functions.rs"));
