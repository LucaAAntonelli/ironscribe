use dioxus::prelude::*;

struct BookRecord {
    book_id: usize,
    title: String,
    authors: Vec<String>,
    series_and_volume: Vec<SeriesAndVolume>,
}

struct SeriesAndVolume {
    series: String,
    volume: f64,
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
pub async fn list_books() -> Result<Vec<(String, String, String, String)>, ServerFnError> {
    let books = DB.with(|f| {
        f.prepare("SELECT b.title, json_group_array(a.name) authors, s.name  AS series, bsl.entry FROM books b JOIN books_authors_link bal ON bal.book = b.id JOIN authors a ON a.id = bal.author LEFT JOIN books_series_link bsl ON bsl.book = b.id LEFT JOIN series s ON s.id = bsl.series GROUP BY b.id, b.title, s.name, bsl.entry ORDER BY b.date_added ASC")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2).unwrap_or_default(), match row.get::<_, Option<f64>>(3) {
                Ok(Some(f)) => f.to_string(),
                _ => "".to_string()
            })))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });
    Ok(books)
}
