#![cfg(feature = "server")]
use crate::database::connection::with_conn;
use crate::database::models::{BookRecord, BookRecords};
use anyhow::anyhow;
use dioxus::prelude::server_fn::error::NoCustomError;
use dioxus::prelude::*;

#[server]
pub async fn list_books() -> Result<BookRecords, ServerFnError> {
    let query = r#"WITH series_info AS (
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
                books.date_added ASC"#;
    let books = with_conn(|conn| {
        let mut stmt = conn.prepare(query)?;
        let rows = stmt.query_map([], |row| {
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
        })?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|_| anyhow!("Failed to map SQL data to Rust structs!"))
    }).map_err(|e| -> ServerFnError<NoCustomError> { ServerFnError::ServerError(e.to_string()) })?;
    Ok(BookRecords { records: books })
}
