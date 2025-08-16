-- Keep for now to not break UI parts from tutorial
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
