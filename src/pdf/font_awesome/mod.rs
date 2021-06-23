mod fonts;
pub use fonts::*;
use printpdf::*;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct UnkownFontError {
    font: String,
}

impl UnkownFontError {
    fn new(font: &str) -> Self {
        Self {
            font: font.to_string(),
        }
    }
}

impl fmt::Display for UnkownFontError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown font: {}", self.font)
    }
}

impl Error for UnkownFontError {
    fn description(&self) -> &str {
        Box::leak(self.to_string().into_boxed_str())
    }

    fn cause(&self) -> Option<&(dyn Error)> {
        None
    }
}

pub struct FontAwesome {
    regular: IndirectFontRef,
}

impl FontAwesome {
    pub fn new(regular: IndirectFontRef) -> Self {
        Self { regular }
    }
    pub fn print_icon(
        &self,
        current_layer: PdfLayerReference,
        name: &str,
        font_size: i64,
        color: Color,
    ) -> Result<(), Box<dyn Error>> {
        let icon = FONTS.get(name).ok_or(UnkownFontError::new(name))?;
        let font = &self.regular;
        current_layer.set_fill_color(color);
        current_layer.set_font(&font, font_size as f64);
        current_layer.write_text(*icon, &font);
        Ok(())
    }
}
