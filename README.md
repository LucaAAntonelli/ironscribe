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
- Expand functionality: So far, books can be uploaded and downloaded via REST, but only with simple `cURL` commands and with synthetic book data.
    - Both currently don't really offer any methods to e.g, upload only works with hand-typed JSON data, download just returns a JSON object with the serialized file content
    - Replace `BookStore` hashmap with proper SQL database for persistent
        - Ideally make it easil switchable for quick debugging and working without access to the database
    - Look into how to properly let files be downloaded via `cURL`
    - Add a getter method for a list of all books
    - Work through exact data stored in the `Book` struct
        - E.g., in the future, multiple authors per book, metadata like number of pages, cover image file etc.
        - Think about how to store the data (one single struct, multiple structs with `Into` and `From` traits implemented for conversion, ...)
- Implement upload and download via gRPC as seen [here](https://github.com/optimumood/grpc-file-transfer-rust)