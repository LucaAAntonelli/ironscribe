use dioxus::prelude::*;
#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("hotdog.db").expect("Failed to open database");

        let sql = std::fs::read_to_string("schema.sql").expect("Failed to parse SQL schema file!");
        conn.execute("PRAGMA foreign_keys = ONE", []).unwrap();
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
        f.prepare("SELECT b.title, a.name, s.name, bsl.entry FROM books b JOIN books_authors_link bal ON bal.book = b.id JOIN authors a ON a.id = bal.author JOIN books_series_link bsl ON bsl.book = b.id JOIN series s ON s.id = bsl.series ORDER BY b.date_added ASC")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get::<_, f64>(3)?.to_string())))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });
    Ok(books)
}
