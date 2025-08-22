use chrono::{DateTime, NaiveDateTime, Utc};
use std::{cmp::Ordering, fmt::Display};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BookRecords {
    pub records: Vec<BookRecord>,
}

impl BookRecords {
    pub fn sort(&mut self, sort_key: SortKey, ascending: bool) -> Self {
        self.records.sort_by(|a, b| match sort_key {
            SortKey::Title => a.sort.cmp(&b.sort),
            SortKey::Author => a.authors_sort[0].cmp(&b.authors_sort[0]),
            SortKey::SeriesAndVolume => {
                match (a.series_and_volume.first(), b.series_and_volume.first()) {
                    (Some(sa), Some(sb)) => {
                        // Both records have a series
                        sa.sort
                            .cmp(&sb.sort)
                            .then_with(|| sa.volume.total_cmp(&sb.volume))
                    }
                    (None, Some(_)) => Ordering::Less,
                    (Some(_), None) => Ordering::Greater,
                    (None, None) => Ordering::Equal,
                }
            }
            SortKey::DateAdded => a.date_added.cmp(&b.date_added),
            SortKey::DatePublished => a.date_published.cmp(&b.date_published),
            SortKey::NumberOfPages => a.number_of_pages.cmp(&b.number_of_pages),
        });
        if !ascending {
            self.records.reverse();
        }
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BookRecord {
    book_id: usize,
    title: String,
    pub sort: String,
    authors: Vec<String>,
    pub authors_sort: Vec<String>,
    pub series_and_volume: Vec<SeriesAndVolume>,
    pub number_of_pages: u32,
    goodreads_id: u64,
    pub date_added: DateTime<Utc>,
    pub date_published: DateTime<Utc>,
    date_modified: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SeriesAndVolume {
    series: String,
    pub sort: String,
    volume: f64,
}

#[derive(PartialEq, Clone)]
pub enum SortKey {
    Title,
    Author,
    SeriesAndVolume,
    DateAdded,
    DatePublished,
    NumberOfPages,
}

impl Display for SeriesAndVolume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} #{}", self.series, self.volume)
    }
}

impl BookRecord {
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_authors(&self) -> Vec<String> {
        self.authors.clone()
    }

    pub fn get_series_and_volumes(&self) -> Vec<SeriesAndVolume> {
        self.series_and_volume.clone()
    }

    pub fn get_pages(&self) -> u32 {
        self.number_of_pages
    }

    pub fn get_date_added(&self) -> DateTime<Utc> {
        self.date_added
    }

    pub fn get_date_published(&self) -> DateTime<Utc> {
        self.date_published
    }
}

pub fn sort_books(
    books: &mut Vec<BookRecord>,
    sort_key: SortKey,
    ascending: bool,
) -> Vec<BookRecord> {
    books.sort_by(|a, b| match sort_key {
        SortKey::Title => a.sort.cmp(&b.sort),
        SortKey::Author => a.authors_sort[0].cmp(&b.authors_sort[0]),
        SortKey::SeriesAndVolume => {
            match (a.series_and_volume.first(), b.series_and_volume.first()) {
                (Some(sa), Some(sb)) => {
                    // Both records have a series
                    sa.sort
                        .cmp(&sb.sort)
                        .then_with(|| sa.volume.total_cmp(&sb.volume))
                }
                (None, Some(_)) => Ordering::Less,
                (Some(_), None) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            }
        }
        SortKey::DateAdded => a.date_added.cmp(&b.date_added),
        SortKey::DatePublished => a.date_published.cmp(&b.date_published),
        SortKey::NumberOfPages => a.number_of_pages.cmp(&b.number_of_pages),
    });
    if !ascending {
        books.reverse();
    }
    books.to_owned()
}

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("hotdog.db").expect("Failed to open database");

        let sql = std::fs::read_to_string("schema.sql").expect("Failed to parse SQL schema file!");
        conn.execute("PRAGMA foreign_keys = ON", []).unwrap();
        conn.execute_batch(&sql).unwrap();

        conn
    };
}

#[server]
pub async fn save_dog(image: String) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("INSERT INTO dogs (url) VALUES (?1)", &[&image]))?;

    Ok(())
}

#[server]
pub async fn list_dogs() -> Result<Vec<(usize, String)>, ServerFnError> {
    let dogs = DB.with(|f| {
        f.prepare("SELECT id, url FROM dogs ORDER BY id DESC LIMIT 10")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });

    Ok(dogs)
}

#[server]
pub async fn list_books() -> Result<BookRecords, ServerFnError> {
    let books = DB.with(|f| {
        f.prepare(
            "
            WITH series_info AS (
                SELECT 
                    bsl.book, 
                    json_group_array(
                        json_object(
                            'series', s.name, 'sort', s.sort, 'volume', bsl.entry
                        )
                    ) series_and_volume 
                FROM 
                    series AS s 
                    JOIN books_series_link bsl ON bsl.series = s.id 
                GROUP BY 
                    bsl.book
            ), 
            authors_info AS (
                SELECT 
                    json_group_array(a.name) authors, 
                    json_group_array(a.sort) authors_sort,
                    bal.book 
                FROM 
                    authors AS a 
                    JOIN books_authors_link bal ON a.id = bal.author 
                GROUP BY 
                    bal.book
            ) 
            SELECT 
                id, title, sort, date_added, date_published, last_modified, number_of_pages, goodreads_id, authors, authors_sort, series_and_volume 
            FROM 
                books 
                LEFT JOIN series_info ON series_info.book = books.id 
                JOIN authors_info ON authors_info.book = books.id 
            ORDER BY 
                books.date_added ASC
        ",
        )
        .unwrap()
        .query_map([], |row| {
            let authors_json_str: String = row.get("authors")?;
            let authors_sort_json_str: String = row.get("authors_sort")?;
            let series_json_str: String = row.get("series_and_volume").unwrap_or_default();
            Ok(BookRecord {
                book_id: row.get("id")?,
                title: row.get("title")?,
                sort: row.get("sort")?,
                authors: serde_json::from_str(&authors_json_str).unwrap(),
                authors_sort: serde_json::from_str(&authors_sort_json_str).unwrap(),
                series_and_volume: serde_json::from_str(&series_json_str).unwrap_or(vec![]),
                number_of_pages: row.get("number_of_pages")?,
                date_added: row.get("date_added")?,
                date_published: row.get("date_published")?,
                date_modified: row.get("last_modified")?,
                goodreads_id: row.get("goodreads_id")?,

            })
        })
        .unwrap()
        .map(|r| r.unwrap())
        .collect()
    });
    Ok(BookRecords { records: books })
}
