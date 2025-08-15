use dioxus::prelude::*;
#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("hotdog.db").expect("Failed to open database");

        conn.execute_batch("
            CREATE TABLE IF NOT EXISTS dogs (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS authors (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                sort TEXT NOT NULL,
                goodreads_link TEXT
            );
            CREATE TABLE IF NOT EXISTS books (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                sort TEXT NOT NULL,
                date_added TIMESTAMP NOT NULL,
                date_published TIMESTAMP,
                last_modified TIMESTAMP,
                number_of_pages INTEGER NOT NULL,
                goodreads_link TEXT
            );
            CREATE TABLE IF NOT EXISTS series (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                sort TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS read_books (
                book INTEGER NOT NULL,
                start_date TIMESTAMP NOT NULL,
                end_date TIMESTAMP,
                FOREIGN KEY(book) REFERENCES books(id)
            );
            CREATE TABLE IF NOT EXISTS books_authors_link (
                book INTEGER NOT NULL,
                author INTEGER NOT NULL,
                FOREIGN KEY(book) REFERENCES books(id),
                FOREIGN KEY(author) REFERENCES authors(id)
            );
            CREATE TABLE IF NOT EXISTS books_series_link (
                book INTEGER NOT NULL,
                series INTEGER NOT NULL, 
                entry REAL NOT NULL,
                FOREIGN KEY(book) REFERENCES books(id),
                FOREIGN KEY(series) REFERENCES series(id)
            );
        ").unwrap();

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
