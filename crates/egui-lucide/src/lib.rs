use egui::{Color32, Image, ImageSource, Response, Widget};
use lucide_rs::{Color, Icon as InnerIcon};

pub struct Icon {
    inner: InnerIcon,
}

impl Icon {
    pub fn size(mut self, size: u32) -> Self {
        self.inner = self.inner.size(size);
        self
    }

    pub fn color(mut self, color: &Color32) -> Self {
        self.inner = self
            .inner
            .color(Color::new(color.r(), color.g(), color.b()));
        self
    }

    pub fn stroke_width(mut self, stroke_width: u32) -> Self {
        self.inner = self.inner.stroke_width(stroke_width);
        self
    }

    pub fn ui(self, ui: &mut egui::Ui) -> Response {
        let bytes = self.inner.svg().into_bytes();
        let image = Image::from_bytes(
            format!(
                "bytes://{}_{}_{}_{}.svg",
                self.inner.name, self.inner.size, self.inner.stroke_width, self.inner.color
            ),
            bytes,
        )
        .fit_to_original_size(1.0);
        image.ui(ui)
    }
}

include!(concat!(env!("OUT_DIR"), "/icon_impls.rs"));
