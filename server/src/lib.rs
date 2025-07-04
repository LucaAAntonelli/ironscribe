use tonic::{Request, Response, Status, Streaming};
use shared::shared::{dir_sync_server::DirSync, Block, ChecksumRequest, ChecksumResponse, DiffRequest, DiffResponse, HelloRequest, HelloResponse, SyncRequest, SyncResponse, UploadResponse};
use std::{collections::HashMap, sync::RwLock};

#[derive(Debug, Default)]
pub struct MyDirSync {
    path_to_checksum: RwLock<HashMap<String, String>>,
    absolute_directory: String,
}

impl MyDirSync {
    fn copy_existing(&self, abs_path: String, incoming_checksum: String) -> bool {
        // Lock server mutex
        
        // Loop over map of path -> checksums
            // If checksum doesn't match incoming, skipped

            // If checksum matches incoming, force-copy from path to abs_path 
            // return true
            //

        // Unlock server mutex
        return false;

    }

    fn update_checksum(&self, path: String, checksum: String)  {
        let mut map = self.path_to_checksum.write().unwrap();
        map.insert(path, checksum);
    }

    fn delete_checksum(&self, path: String) {
        let mut map = self.path_to_checksum.write().unwrap();
        map.remove(&path);
    }
}

#[tonic::async_trait]
impl DirSync for MyDirSync {
    
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let response = HelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(response))
    }

    async fn sync_structure(
        &self,
        request: Request<SyncRequest>,
    ) -> Result<Response<SyncResponse>, Status> {
        // Create key-value pairs for all elements in SyncRequest.get_elements()

        // Iterate over all elements

            // Get path out of element, sanitize path and join with server's absolute directory
            
            // Add path to key-value store
            
            // If element is a directory, create the directory 

        // Recursively walk over file tree from server's absolute directory (root is skipped!)

            // If file doesn't exist in key-value pairs, delete the file
            
            // If file exists in key-value pairs but not in the directory, it was already deleted
            // => Remove from key-value pairs
            
            // If an error occurred (permissions, busy file etc.), return with an error 
            
        // SUMMARY: Created a map of paths and checksums from elements in request. Then, recursively
        // walk the server's root directory and for every file/folder, if there is no entry in the
        // map, remove it. If there is no file in the directory, remove the map entry
        todo!("IMPLEMENT sync_structure()!");
    }

    async fn diff_structure(
        &self,
        request: Request<DiffRequest>,
    ) -> Result<Response<DiffResponse>, Status> {
        // Iterate over all created elements from request, assemble path, create directory if it is
        // a directory

        // Iterate over all deleted elements from request, assemble path, remove directory if it is
        // a directory, delete checksum entry

        // SUMMARY: Create all directories from request.getCreated(), delete all directories from
        // request.getDeleted()
        todo!("IMPLEMENT diff_structure()!");
    }

    async fn get_checksum(
        &self,
        request: Request<ChecksumRequest>,
    ) -> Result<Response<ChecksumResponse>, Status> {
        // Get path from request and create checksum for it (joined with root path)


        todo!("IMPLEMENT get_checksum()!");
    }

    async fn upload_blocks(
        &self,
        request: Request<Streaming<Block>>,
    ) -> Result<Response<UploadResponse>, Status> {
        todo!("IMPLEMENT upload_blocks()!");
    }
}


