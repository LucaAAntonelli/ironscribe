# IronScribe
A server-client application for ebook syncing and management written in Rust

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
- [ ] Port sync functionality from [here](https://github.com/FZambia/dirsync)
  - [ ] Server
  - [ ] Client
- [ ] Once done, look for possible improvements
  - [ ] Usage with ebook files -> folder structure likely similar to Calibre
  - [ ] Books will likely be added either via GUI or by automatically copying from a specified folder, also similar to Calibre
  - [ ] Because files will change only through GUI except previous point, no need for complex request-response for sync. Instead, create individual services for adding, editing and deleting
- Look into short-comings of original repo, such as 
    - not working on Windows file system
    - only working from client to server, not bidirectionally
    - only working with a single client (might be necessary for web access later)

# Current Plan for data
Split the file data and metadata of all books. There will be at least two structs; one being the file itself (name, path, data as bytes etc) and one being the book metadata (title, authors, series, index, publication date, pages, ...). Metadata will be automatically extracted from a book's EPUB file and may be stored in a separate file similar to Calibre. 

In order to use this approach, both REST and gRPC need to provide a service for upload and download relating to the first struct, and a service to interact with the metadata (e.g., to get book metadata so it can be displayed in a client). As such, the protobuf file needs to be adapted to offer both of those structs. 
