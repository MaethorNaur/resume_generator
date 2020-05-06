mod font_awesome;
mod image;
mod shape;
mod text;
mod timeline;

use crate::resume::{Language, Location, Resume, Skill};
use chrono::prelude::*;
use font_awesome::FontAwesome;
use printpdf::utils::calculate_points_for_circle;
use printpdf::*;
use std::convert::From;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Cursor};
use std::path::PathBuf;
use timeline::*;

const FONT_REGULAR: &[u8] = include_bytes!("../fonts/liberation.ttf");
const FONT_BOLD: &[u8] = include_bytes!("../fonts/liberation-bold.ttf");
const FONT_THIN: &[u8] = include_bytes!("../fonts/liberation-thin.ttf");

const DOC_WIDTH: Mm = Mm(210.0);
const DOC_HEIGHT: Mm = Mm(297.0);

const PROFILE_SIZE: Mm = Mm(74.5);
const PROFILE_X_OFFSET: Mm = Mm(5.);
const PROFILE_Y_OFFSET: Mm = Mm(PROFILE_SIZE.0);

const RADIUS: Pt = Pt(5.);

const LEFT_COLUMN_SIZE: Mm = PROFILE_SIZE;
pub(super) const RIGHT_COLUMN_HEIGHT: Mm = Mm(74.5);

const DPI: f64 = 300.0;

const INFO: &str = "Info";
const LANGUAGES: &str = "Spoken languages";
const SOCIALS: &str = "Social";
const SKILLS: &str = "Skills";

const DATE_FORMAT: &str = "%b %Y";

pub struct Pdf {
    resume: Resume,
    font_awesome: FontAwesome,
    font_regular: IndirectFontRef,
    font_bold: IndirectFontRef,
    font_thin: IndirectFontRef,
    layer: PdfLayerReference,
    doc: PdfDocumentReference,
    primary_color: Color,
    secondary_color: Color,
}

impl Pdf {
    pub fn new(resume: Resume) -> Result<Self, Box<dyn Error>> {
        let (doc, page1, layer1) = PdfDocument::new("Resume", DOC_WIDTH, DOC_HEIGHT, "Layer 1");
        let font_regular = doc.add_external_font(Cursor::new(FONT_REGULAR.as_ref()))?;
        let font_bold = doc.add_external_font(Cursor::new(FONT_BOLD.as_ref()))?;
        let font_thin = doc.add_external_font(Cursor::new(FONT_THIN.as_ref()))?;

        let font_awesome = FontAwesome::new(font_regular.clone());

        let layer = doc.get_page(page1).get_layer(layer1);

        Ok(Self {
            resume,
            font_awesome,
            font_regular,
            font_bold,
            font_thin,
            layer,
            doc,
            primary_color: Color::Cmyk(Cmyk::new(0.78, 0.62, 0.66, 0.71, None)),
            secondary_color: Color::Rgb(Rgb::new(1., 1., 1., None)),
        })
    }

    pub fn save(self, filename: &PathBuf) -> Result<(), Box<dyn Error>> {
        debug!("Generating pdf: {:?}", filename);
        self.draw_left_background();
        let start = self.add_profile_picture();

        self.social_qr_code()?;
        self.write_bio(start);

        self.write_info(start)?;
        self.write_social(start);
        self.write_languages(start);
        self.write_skills(start);

        self.write_timeline();
        self.doc
            .save(&mut BufWriter::new(File::create(filename)?))?;
        Ok(())
    }

    fn write_bio(&self, start: Mm) {
        self.layer
            .set_fill_color(Color::Rgb(Rgb::new(1., 1., 1., None)));
        let offset_x = PROFILE_X_OFFSET;
        let offset_y = DOC_HEIGHT - (start + Mm(10.));

        self.layer.begin_text_section();
        self.layer.set_font(&self.font_bold, 20);
        self.layer.set_text_cursor(offset_x, offset_y);
        self.layer.set_line_height(18);
        self.resume
            .basics
            .name
            .to_uppercase()
            .split_whitespace()
            .for_each(|part| {
                self.layer.write_text(part, &self.font_bold);
                self.layer.add_line_break();
            });

        self.layer.set_font(&self.font_regular, 10);
        self.layer
            .write_text(&self.resume.basics.label, &self.font_regular);
        self.layer.end_text_section();
    }

    fn write_info(&self, start: Mm) -> Result<(), Box<dyn Error>> {
        self.layer.set_fill_color(self.secondary_color.clone());
        let offset_x = PROFILE_X_OFFSET;
        let offset_y = DOC_HEIGHT - (start + Mm(35.));

        self.layer.begin_text_section();
        self.layer.set_line_height(18);
        self.write_underlined_text(INFO, 12, offset_x, offset_y);
        self.layer.add_line_break();
        self.layer.set_line_height(16);

        self.layer.add_line_break();

        self.font_awesome.print_icon(
            self.layer.clone(),
            "envelope",
            12,
            self.secondary_color.clone(),
        )?;

        self.layer.set_font(&self.font_regular, 9);
        self.layer.write_text(
            format!("   {}", &self.resume.basics.email),
            &self.font_regular,
        );

        if let Some(phone) = &self.resume.basics.phone {
            self.layer.add_line_break();
            self.font_awesome.print_icon(
                self.layer.clone(),
                "phone",
                12,
                self.secondary_color.clone(),
            )?;
            self.layer.set_font(&self.font_regular, 9);
            self.layer
                .write_text(format!("   {}", phone), &self.font_regular);
        }
        if let Some(Location {
            city: Some(city),
            country_code,
            ..
        }) = &self.resume.basics.location
        {
            self.layer.add_line_break();
            self.font_awesome.print_icon(
                self.layer.clone(),
                "map-marker",
                12,
                self.secondary_color.clone(),
            )?;
            self.layer.set_font(&self.font_regular, 9);
            let text = match country_code.as_ref() {
                None => String::new(),
                Some(country_code) => format!(", {}", country_code),
            };
            self.layer
                .write_text(format!("   {}{}", city, text), &self.font_regular);
        }

        if let Some(birthday) = &self.resume.basics.birthday {
            self.layer.add_line_break();
            self.font_awesome.print_icon(
                self.layer.clone(),
                "cake",
                12,
                self.secondary_color.clone(),
            )?;
            let age = Local::now()
                .naive_utc()
                .date()
                .signed_duration_since(*birthday)
                .num_weeks()
                / 52;
            self.layer.set_font(&self.font_regular, 9);
            self.layer.write_text(
                format!(
                    "    {} ({}yo)",
                    birthday.format("%d %b %Y").to_string(),
                    age
                ),
                &self.font_regular,
            );
        }
        self.layer.end_text_section();
        Ok(())
    }

    fn write_social(&self, start: Mm) {
        self.layer.begin_text_section();
        self.layer.set_fill_color(self.secondary_color.clone());
        let offset_x = PROFILE_X_OFFSET;
        let offset_y = DOC_HEIGHT - (start + Mm(25.) + Mm(48.));
        self.layer.set_line_height(18);
        self.write_underlined_text(SOCIALS, 12, offset_x, offset_y);
        self.layer.add_line_break();
        self.layer.set_line_height(16);

        self.resume.basics.profiles.iter().for_each(|profile| {
            self.layer.add_line_break();
            let network = profile.network.to_lowercase();
            self.write_social_icon(&network, 12).unwrap();
            self.layer.set_fill_color(self.secondary_color.clone());
            self.layer.set_font(&self.font_regular, 10);
            self.layer
                .write_text(format!("   {}", &profile.username), &self.font_regular);
        });

        self.layer.end_text_section();
    }

    fn write_languages(&self, start: Mm) {
        self.layer.begin_text_section();
        self.layer.set_fill_color(self.secondary_color.clone());
        let offset_x = PROFILE_X_OFFSET;
        let offset_y = DOC_HEIGHT - (start + Mm(25.) + Mm(85.));
        self.layer.set_line_height(18);

        self.write_underlined_text(LANGUAGES, 12, offset_x, offset_y);

        self.layer.add_line_break();

        self.layer.set_line_height(16);
        let width = self
            .resume
            .languages
            .iter()
            .map(|l| l.language.len())
            .max()
            .unwrap_or(0)
            + 4;

        self.resume
            .languages
            .iter()
            .for_each(|Language { language, fluency }| {
                self.layer.add_line_break();
                self.layer.set_font(&self.font_bold, 10);
                self.layer.write_text(
                    format!("- {:<width$}", language, width = width),
                    &self.font_bold,
                );
                if let Some(fluency) = fluency {
                    self.layer.set_font(&self.font_thin, 10);
                    self.layer.write_text(fluency, &self.font_thin);
                }
            });
        self.layer.end_text_section();
    }

    fn write_skills(&self, start: Mm) {
        self.layer.begin_text_section();
        self.layer.set_fill_color(self.secondary_color.clone());
        let offset_x = PROFILE_X_OFFSET;
        let offset_y = DOC_HEIGHT - (start + Mm(25.) + Mm(85.) + Mm(40.));
        self.layer.set_line_height(18);

        self.write_underlined_text(SKILLS, 12, offset_x, offset_y);

        self.layer.add_line_break();
        self.layer.set_line_height(14);
        self.layer.add_line_break();
        self.resume
            .skills
            .iter()
            .for_each(|Skill { name, keywords, .. }| {
                self.layer.set_font(&self.font_bold, 10);
                self.layer
                    .write_text(format!("- {}", name), &self.font_bold);
                self.layer.add_line_break();
                self.layer.set_font(&self.font_thin, 10);
                self.write_bounded(&keywords.join("  -  "), 36);
            });
        self.layer.end_text_section();
    }

    fn write_timeline(&self) {
        let mut timeline = Timeline::new();

        self.resume.work.iter().cloned().for_each(|work| {
            let event = Event::from(work);
            timeline.add(event);
        });

        self.resume.education.iter().cloned().for_each(|education| {
            let event = Event::from(education);
            timeline.add(event);
        });

        let events = timeline.events();
        let offset_x: Pt = (LEFT_COLUMN_SIZE + Mm((DOC_WIDTH.0 - LEFT_COLUMN_SIZE.0) / 2.)).into();
        let height: Pt = DOC_HEIGHT.into();
        let top_y: Pt = height - Pt(20.);

        let event_height: Pt = Pt(top_y.0 / events.len() as f64);

        let line_height = Pt(((events.len() - 1) as f64) * event_height.0);

        let line = Line {
            points: shape::rectangle_points(offset_x, top_y - line_height, Pt(2.), line_height),
            is_closed: true,
            has_fill: true,
            has_stroke: false,
            is_clipping_path: false,
        };
        self.layer.set_fill_color(self.primary_color.clone());
        self.layer.add_shape(line);
        let mut previous: Option<&str> = None;
        events.iter().enumerate().for_each(|(i, event)| {
            let indexed_event = (i, event);
            let is_same_has_previous_event =
                previous.map(|p| p == event.institution).unwrap_or(false);

            self.write_event(
                indexed_event,
                is_same_has_previous_event,
                offset_x,
                top_y,
                event_height,
            );
            previous = Some(&event.institution)
        });
    }

    fn add_profile_picture(&self) -> Mm {
        self.resume
            .basics
            .picture
            .as_ref()
            .map(|picture| match image::from_path(picture) {
                Err(_) => {
                    warn!("Picture: {:?} not found", &picture);
                    Mm(15.)
                }
                Ok(image) => {
                    let pt_size: Pt = PROFILE_SIZE.into();

                    let scale_x = pt_size.0 / image.image.width.into_pt(DPI).0;
                    let scale_y = pt_size.0 / image.image.height.into_pt(DPI).0;

                    image.add_to_layer(
                        self.layer.clone(),
                        Some(Mm(0.)),
                        Some(DOC_HEIGHT - PROFILE_Y_OFFSET),
                        None,
                        Some(scale_x),
                        Some(scale_y),
                        Some(DPI),
                    );
                    RIGHT_COLUMN_HEIGHT
                }
            })
            .unwrap_or(Mm(15.))
    }

    fn write_event(
        &self,
        indexed_event: (usize, &Event),
        is_same_has_previous_event: bool,
        offset_x: Pt,
        offset_y: Pt,
        height: Pt,
    ) {
        let (i, event) = indexed_event;
        let pos_y = Pt(offset_y.0 - (i as f64 * height.0));

        let outer_circle = Line {
            points: calculate_points_for_circle(RADIUS, offset_x + Pt(1.), pos_y),
            is_closed: true,
            has_fill: true,
            has_stroke: true,
            is_clipping_path: false,
        };

        self.layer.set_outline_color(self.primary_color.clone());
        if is_same_has_previous_event {
            self.layer.set_fill_color(self.primary_color.clone());
        } else {
            match event.event_type {
                EventType::Work => {
                    self.layer.set_fill_color(self.secondary_color.clone());
                    self.layer.add_shape(outer_circle);

                    let inner_circle = Line {
                        points: calculate_points_for_circle(Pt(1.), offset_x + Pt(1.), pos_y),
                        is_closed: true,
                        has_fill: true,
                        has_stroke: true,
                        is_clipping_path: false,
                    };
                    self.layer.set_fill_color(self.primary_color.clone());
                    self.layer.add_shape(inner_circle);
                }
                EventType::Education => {
                    self.layer.set_fill_color(self.primary_color.clone());
                    self.layer.add_shape(outer_circle);
                }
            }
        }
        self.layer.begin_text_section();

        self.layer.set_font(&self.font_bold, 12);
        self.layer.set_line_height(12);

        self.layer
            .set_text_cursor(LEFT_COLUMN_SIZE + Mm(5.), (pos_y - Pt(20.)).into());

        let end_date = event
            .end_date
            .map(|end_date| end_date.format(DATE_FORMAT).to_string())
            .unwrap_or_else(|| "Today".to_string());

        if !is_same_has_previous_event {
            let mut split_iter = event.institution.split(',');
            let institution = split_iter.next().unwrap();
            let location = split_iter.collect::<Vec<_>>().join(",");
            self.write_bounded(institution, 30);
            self.layer.set_font(&self.font_regular, 9);
            if !location.is_empty() {
                self.write_bounded(&location.trim(), 30);
            }
        };

        self.layer.set_font(&self.font_regular, 9);
        self.write_bounded(&event.label, 30);
        self.layer.set_font(&self.font_thin, 9);
        self.layer.write_text(
            format!("{} - {}", event.start_date.format(DATE_FORMAT), end_date),
            &self.font_regular,
        );

        self.layer.end_text_section();

        self.layer.begin_text_section();

        let offset: Mm = offset_x.into();

        self.layer
            .set_text_cursor(offset + Mm(7.), (pos_y - Pt(20.)).into());
        self.layer.set_font(&self.font_regular, 9);
        if let Some(summary) = &event.summary {
            self.write_bounded(&summary, 36);
        }

        if !event.highlights.is_empty() {
            self.layer.add_line_break();
            self.layer.set_font(&self.font_bold, 9);
            self.write_bounded(&event.highlights.join("  -  "), 35)
        }
        self.layer.end_text_section();
    }

    fn social_qr_code(&self) -> Result<(), Box<dyn Error>> {
        if let Some(url) = &self.resume.basics.website {
            debug!("Generating QRCode for: {:?}", url);
            let qrcode = image::qrcode(&url, 150, image::to_rgb(self.primary_color.clone()))?;
            qrcode.add_to_layer(self.layer.clone(), None, None, None, None, None, Some(DPI));
        }
        Ok(())
    }

    fn draw_left_background(&self) {
        let line = Line {
            points: shape::rectangle_points(
                Pt(0.0),
                Pt(0.0),
                LEFT_COLUMN_SIZE.into(),
                DOC_HEIGHT.into(),
            ),
            is_closed: true,
            has_fill: true,
            has_stroke: false,
            is_clipping_path: false,
        };

        self.layer.set_fill_color(self.primary_color.clone());
        self.layer.add_shape(line);
    }
}
