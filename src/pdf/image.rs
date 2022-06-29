use printpdf::*;
use qrcode_generator::QrCodeEcc;

pub fn from_path(filename: &str) -> Result<Image, image_crate::error::ImageError> {
    let dyn_image = image_crate::open(filename)?;
    Ok(Image::from_dynamic_image(&dyn_image))
}

pub fn qrcode(url: &str, size: usize, color: Rgb) -> Result<Image, image_crate::error::ImageError> {
    let image = qrcode_generator::to_image_buffer_from_str(url, QrCodeEcc::High, size).unwrap();
    let mut buffer = image_crate::DynamicImage::ImageLuma8(image).to_rgb8();
    let new_color = [
        (color.r * 255.0) as u8,
        (color.g * 255.0) as u8,
        (color.b * 255.0) as u8,
    ];
    for pixel in buffer.pixels_mut() {
        if pixel.0 == [0, 0, 0] {
            (*pixel).0 = new_color;
        }
    }
    let dyn_image = image_crate::DynamicImage::ImageRgb8(buffer);
    Ok(Image::from_dynamic_image(&dyn_image))
}

pub fn to_rgb(color: Color) -> Rgb {
    match color {
        Color::Rgb(rgb) => rgb,
        Color::Cmyk(Cmyk { c, m, k, y, .. }) => cmyk_to_rgb(c, m, y, k),
        Color::Greyscale(grey_scale) => Rgb::new(
            grey_scale.percent,
            grey_scale.percent,
            grey_scale.percent,
            None,
        ),
        Color::SpotColor(SpotColor { c, m, y, k, .. }) => cmyk_to_rgb(c, m, y, k),
    }
}

fn cmyk_to_rgb(c: f64, m: f64, y: f64, k: f64) -> Rgb {
    Rgb::new(
        (1. - c) * (1. - k),
        (1. - m) * (1. - k),
        (1. - y) * (1. - k),
        None,
    )
}
