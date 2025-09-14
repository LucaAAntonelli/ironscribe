use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Clone)]
pub struct BookRecords { pub records: Vec<BookRecord> }

impl BookRecords {
    pub fn sort(&mut self, sort_key: SortKey, ascending: bool) -> Self {
        self.records.sort_by(|a, b| match sort_key {
            SortKey::Title => a.sort.cmp(&b.sort),
            SortKey::Author => a.authors_sort[0].cmp(&b.authors_sort[0]),
            SortKey::SeriesAndVolume => match (a.series_and_volume.first(), b.series_and_volume.first()) {
                (Some(sa), Some(sb)) => sa.sort.cmp(&sb.sort).then_with(|| sa.volume.total_cmp(&sb.volume)),
                (None, Some(_)) => Ordering::Less,
                (Some(_), None) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            },
            SortKey::DateAdded => a.date_added.cmp(&b.date_added),
            SortKey::DatePublished => a.date_published.cmp(&b.date_published),
            SortKey::NumberOfPages => a.number_of_pages.cmp(&b.number_of_pages),
        });
        if !ascending { self.records.reverse(); }
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BookRecord {
    pub(crate) book_id: usize,
    pub(crate) title: String,
    pub(crate) sort: String,
    pub(crate) authors: Vec<String>,
    pub(crate) authors_sort: Vec<String>,
    pub(crate) series_and_volume: Vec<SeriesAndVolume>,
    pub(crate) number_of_pages: u32,
    pub(crate) goodreads_id: u64,
    pub(crate) date_added: DateTime<Utc>,
    pub(crate) date_published: DateTime<Utc>,
    pub(crate) date_modified: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SeriesAndVolume { pub series: String, pub sort: String, pub volume: f64 }

#[derive(PartialEq, Clone)]
pub enum SortKey { Title, Author, SeriesAndVolume, DateAdded, DatePublished, NumberOfPages }

impl Display for SeriesAndVolume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{} #{}", self.series, self.volume) }
}

impl BookRecord {
    pub fn get_title(&self) -> String { self.title.clone() }
    pub fn get_authors(&self) -> Vec<String> { self.authors.clone() }
    pub fn get_series_and_volumes(&self) -> Vec<SeriesAndVolume> { self.series_and_volume.clone() }
    pub fn get_pages(&self) -> u32 { self.number_of_pages }
    pub fn get_date_added(&self) -> DateTime<Utc> { self.date_added }
    pub fn get_date_published(&self) -> DateTime<Utc> { self.date_published }
}
