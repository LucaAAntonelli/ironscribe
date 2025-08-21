use chrono::{DateTime, NaiveDateTime, Utc};
use std::fmt::Display;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BookRecord {
    book_id: usize,
    title: String,
    pub sort: String,
    authors: Vec<String>,
    pub authors_sort: Vec<String>,
    pub series_and_volume: Vec<SeriesAndVolume>,
    number_of_pages: u32,
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
}

pub fn sort_books(
    books: &mut Vec<BookRecord>,
    sort_key: SortKey,
    ascending: bool,
) -> Vec<BookRecord> {
    books.sort_by_key(|k| match sort_key {
        SortKey::Title => k.sort.clone(),
        SortKey::Author => k.authors_sort[0].clone(),
        SortKey::SeriesAndVolume => k
            .series_and_volume
            .first()
            .map_or(String::from(""), |sv| sv.sort.clone()),
        SortKey::DateAdded => k.date_added.to_rfc3339(),
        SortKey::DatePublished => k.date_published.to_rfc3339(),
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
pub async fn list_books() -> Result<Vec<BookRecord>, ServerFnError> {
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
    Ok(books)
}
