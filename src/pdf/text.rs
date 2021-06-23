use super::{shape, Pdf, RIGHT_COLUMN_HEIGHT};
use printpdf::*;

use std::error::Error;

impl Pdf {
    pub(super) fn write_social_icon(
        &self,
        name: &str,
        font_size: i64,
    ) -> Result<(), Box<dyn Error>> {
        let color = match name {
            "twitter" => Color::Rgb(Rgb::new(0.11, 0.63, 0.95, None)),
            "linkedin" => Color::Rgb(Rgb::new(0.0, 0.46, 0.70, None)),
            "github" => self.secondary_color.clone(),
            "gitlab" => Color::Rgb(Rgb::new(0.88, 0.26, 0.16, None)),
            _ => self.secondary_color.clone(),
        };
        self.font_awesome
            .print_icon(self.layer.clone(), name, font_size, color)
    }

    pub(super) fn write_underlined_text(
        &self,
        text: &str,
        font_size: i64,
        offset_x: Mm,
        offset_y: Mm,
    ) {
        self.layer.set_font(&self.font_bold, font_size as f64);
        self.layer.set_text_cursor(offset_x, offset_y);
        self.layer.write_text(text, &self.font_bold);
        let offset_y_pt: Pt = offset_y.into();
        let stroke_size = Pt(2.0);
        let line = Line {
            points: shape::rectangle_points(
                offset_x.into(),
                offset_y_pt - (Pt(font_size as f64) + stroke_size),
                (RIGHT_COLUMN_HEIGHT - offset_x - offset_x).into(),
                stroke_size,
            ),
            is_closed: true,
            has_fill: true,
            has_stroke: false,
            is_clipping_path: false,
        };
        self.layer.add_shape(line);
    }

    pub(super) fn write_bounded(&self, text: &str, width: usize) {
        textwrap::fill(text, width).split('\n').for_each(|line| {
            self.layer.write_text(line, &self.font_regular);
            self.layer.add_line_break();
        });
    }
}
