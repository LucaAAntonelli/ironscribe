# IronScribe
A server-client application for ebook syncing and management written in Rust

# Usage (WIP)
Currently, books can be uploaded and downloaded via REST API:

## Upload
Create a JSON file, e.g., `sample.json` with the following structure:
```json
{
    "id": "1",
    "title": "Some Profound Title",
    "author": "Bigshot McGee",
    "format": "epub",
    "content": [
        1,
        2,
        3
    ]
}
```
Then, use the JSON object with the following `cURL` command:

```
curl.exe --request POST http://localhost:8080/upload --header "Content-Type: application/json" --data "@sample.json"
```
The `.exe` extension is needed on Windows because otherwise, PowerShell will use the included `cURL`, whereas `curl.exe` will invoke the version installed via Git Bash.

## Download
To download a book via REST API, simply call the `GET` request with the book's ID:
```
curl.exe --request GET http://localhost:8080/book/1
```
This returns the same JSON object as was used in the `POST` request.

# Goals
- [x] Look into a hybrid server that exposes gRPC and REST, as REST is better usable on the web
- [ ] Create different components
    - [ ] Folder sync
    - [ ] Client/Web GUI
        - [ ] Add/Edit/Remove books
        - [ ] SQL database
        - [ ] Book metadata
    - [ ] Analytics

# Current Tasks:
- Port sync functionality from [here](https://github.com/FZambia/dirsync)
- Once done, look for possible improvements (more idiomatic Rust solutions, improvements based on use case of syncing shallow folder structures, ...)
- Look into shortcommings of original repo, such as 
    - not working on Windows file system
    - only working from client to server, not bidirectionally
    - only working with a single client (might be necessary for web access later)

# Current Plan for data
Split the file data and metadata of all books. There will be at least two structs; one being the file itself (name, path, data as bytes etc) and one being the book metadata (title, authors, series, index, publication date, pages, ...). Metadata will be automatically extracted from a book's EPUB file and may be stored in a separate file similar to Calibre. 

In order to use this approach, both REST and gRPC need to provide a service for upload and download relating to the first struct, and a service to interact with the metadata (e.g., to get book metadata so it can be displayed in a client). As such, the protobuf file needs to be adapted to offer both of those structs. 
