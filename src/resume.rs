use chrono::NaiveDate;
use relative_path::RelativePathBuf;
use serde::{de::Error, Deserialize, Deserializer};
use std::error;
use std::ffi::OsStr;
use std::fs::{read_to_string, File};
use std::io::BufReader;
use std::path::PathBuf;
const FORMAT: &str = "%Y-%m-%d";

#[derive(Debug, Deserialize)]
pub struct Resume {
    pub basics: Basics,
    #[serde(default)]
    pub work: Vec<Work>,
    #[serde(default)]
    pub volunteer: Vec<Volunteer>,
    #[serde(default)]
    pub education: Vec<Education>,
    #[serde(default)]
    pub awards: Vec<Award>,
    #[serde(default)]
    pub publications: Vec<Publication>,
    #[serde(default)]
    pub skills: Vec<Skill>,
    #[serde(default)]
    pub languages: Vec<Language>,
    #[serde(default)]
    pub interests: Vec<Interest>,
    #[serde(default)]
    pub references: Vec<Reference>,
}

#[derive(Debug, Deserialize)]
pub struct Basics {
    pub name: String,
    pub label: String,
    pub picture: Option<String>,
    pub email: String,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub summary: Option<String>,
    pub location: Option<Location>,
    #[serde(default, deserialize_with = "option_date_deserializer")]
    pub birthday: Option<NaiveDate>,
    #[serde(default)]
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub address: Option<String>,
    #[serde(rename(deserialize = "postalCode"))]
    pub postal_code: Option<String>,
    pub city: Option<String>,
    #[serde(rename(deserialize = "countryCode"))]
    pub country_code: Option<String>,
    pub region: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub network: String,
    pub username: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Work {
    pub company: String,
    pub position: String,
    pub website: Option<String>,
    #[serde(
        rename(deserialize = "startDate"),
        deserialize_with = "date_deserializer"
    )]
    pub start_date: NaiveDate,
    #[serde(
        default,
        rename(deserialize = "endDate"),
        deserialize_with = "option_date_deserializer"
    )]
    pub end_date: Option<NaiveDate>,
    pub summary: String,
    #[serde(default)]
    pub highlights: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Volunteer {
    pub organization: String,
    pub position: String,
    pub website: Option<String>,
    #[serde(
        rename(deserialize = "startDate"),
        deserialize_with = "date_deserializer"
    )]
    pub start_date: NaiveDate,
    #[serde(
        default,
        rename(deserialize = "endDate"),
        deserialize_with = "option_date_deserializer"
    )]
    pub end_date: Option<NaiveDate>,
    pub summary: String,
    #[serde(default)]
    pub highlights: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Education {
    pub institution: String,
    pub area: String,
    #[serde(rename(deserialize = "studyType"))]
    pub study_type: String,
    #[serde(
        rename(deserialize = "startDate"),
        deserialize_with = "date_deserializer"
    )]
    pub start_date: NaiveDate,
    #[serde(
        default,
        rename(deserialize = "endDate"),
        deserialize_with = "option_date_deserializer"
    )]
    pub end_date: Option<NaiveDate>,
    pub gpa: Option<String>,
    #[serde(default)]
    pub courses: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Award {
    pub title: String,
    pub date: String,
    pub awarder: String,
    pub summary: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Publication {
    pub name: String,
    pub publisher: String,
    #[serde(
        rename(deserialize = "releaseDate"),
        deserialize_with = "date_deserializer"
    )]
    pub release_date: NaiveDate,
    pub website: Option<String>,
    pub summary: String,
}

#[derive(Debug, Deserialize)]
pub struct Skill {
    pub name: String,
    pub level: Option<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Language {
    pub language: String,
    pub fluency: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Interest {
    pub name: String,
    #[serde(default)]
    pub keywords: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Reference {
    pub name: String,
    pub reference: String,
}

impl Resume {
    pub fn from_path(path: PathBuf) -> Result<Self, Box<dyn error::Error>> {
        debug!("Opening resume: {:?}", path);
        let mut resume: Resume = match path.extension().and_then(OsStr::to_str) {
            Some("toml") => toml::from_str(&read_to_string(&path)?)?,
            _ => {
                let file = File::open(&path)?;
                let reader = BufReader::new(file);
                serde_json::from_reader(reader)?
            }
        };
        if let Some(picture) = &mut resume.basics.picture {
            *picture = RelativePathBuf::from_path(&path.parent().unwrap())?
                .join(&picture)
                .normalize()
                .as_str()
                .to_string();
        }
        Ok(resume)
    }
}

fn option_date_deserializer<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<NaiveDate>, D::Error> {
    #[derive(Deserialize)]
    struct Wrapper(#[serde(deserialize_with = "date_deserializer")] NaiveDate);
    let v = Option::deserialize(deserializer)?;
    Ok(v.map(|Wrapper(a)| a))
}
fn date_deserializer<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDate, D::Error> {
    let time: String = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&time, FORMAT).map_err(D::Error::custom)
}
