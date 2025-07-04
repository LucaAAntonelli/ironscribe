use tonic::{Request, Response, Status, Streaming};
use shared::shared::{dir_sync_server::DirSync, Block, ChecksumRequest, ChecksumResponse, DiffRequest, DiffResponse, HelloRequest, HelloResponse, SyncRequest, SyncResponse, UploadResponse};

#[derive(Debug, Default)]
pub struct MyDirSync {}

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
        // a directory

        // SUMMARY: Create all directories from request.getCreated(), delete all directories from
        // request.getDeleted()
        todo!("IMPLEMENT diff_structure()!");
    }

    async fn get_checksum(
        &self,
        request: Request<ChecksumRequest>,
    ) -> Result<Response<ChecksumResponse>, Status> {
        todo!("IMPLEMENT get_checksum()!");
    }

    async fn upload_blocks(
        &self,
        request: Request<Streaming<Block>>,
    ) -> Result<Response<UploadResponse>, Status> {
        todo!("IMPLEMENT upload_blocks()!");
    }
}


