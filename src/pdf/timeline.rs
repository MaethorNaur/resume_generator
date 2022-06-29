use crate::resume::{Education, Work};

use chrono::NaiveDate;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum EventType {
    Work,
    Education,
}

#[derive(Debug, Clone)]
pub struct Event {
    pub event_type: EventType,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub institution: String,
    pub label: String,
    pub summary: Option<String>,
    pub highlights: Vec<String>,
}

#[derive(Debug)]
pub struct Timeline {
    events: Vec<Event>,
}

impl Timeline {
    pub fn new() -> Self {
        Self { events: vec![] }
    }

    pub fn add(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn events(&self) -> Vec<Event> {
        let mut events: Vec<Event> = self.events.to_vec();
        events.sort_by(|a, b| match (a.end_date, b.end_date) {
            (None, None) => b.start_date.partial_cmp(&a.start_date).unwrap(),
            (None, _) => Ordering::Less,
            (_, None) => Ordering::Greater,
            (Some(a), Some(b)) => b.partial_cmp(&a).unwrap(),
        });
        events
    }
}

impl From<Work> for Event {
    fn from(work: Work) -> Self {
        Self {
            event_type: EventType::Work,
            start_date: work.start_date,
            end_date: work.end_date,
            institution: work.company,
            label: work.position,
            summary: Some(work.summary),
            highlights: work.highlights,
        }
    }
}

impl From<Education> for Event {
    fn from(education: Education) -> Self {
        Self {
            event_type: EventType::Education,
            start_date: education.start_date,
            end_date: education.end_date,
            institution: education.institution,
            label: format!("{} in {}", education.study_type, education.area),
            summary: None,
            highlights: education.courses,
        }
    }
}
