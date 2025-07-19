use shared::filesystem::{
    FileChunker, clean_path, compute_file_sha256, force_copy, walk_filetree_and_apply,
};
use shared::proto::{
    Block, ChecksumRequest, ChecksumResponse, DiffRequest, DiffResponse, SyncRequest, SyncResponse,
    UploadResponse, dir_sync_server::DirSync,
};
use std::fs::{create_dir_all, remove_dir_all, remove_file};
use std::path::PathBuf;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
    sync::RwLock,
};
use tonic::{Request, Response, Status, Streaming};

#[derive(Debug)]
pub struct MyDirSync {
    path_to_checksum: RwLock<HashMap<PathBuf, String>>,
    absolute_directory: PathBuf,
}

impl MyDirSync {
    fn update_checksum(&self, path: PathBuf, checksum: String) {
        let mut map = self.path_to_checksum.write().unwrap();
        map.insert(path, checksum);
    }

    fn delete_checksum(&self, path: PathBuf) {
        let mut map = self.path_to_checksum.write().unwrap();
        map.remove(&path);
    }

    fn get_root_directory(&self) -> &Path {
        &self.absolute_directory
    }

    fn get_file_with_matching_checksum(&self, checksum: String) -> Option<PathBuf> {
        self.path_to_checksum
            .read()
            .unwrap()
            .iter()
            .find_map(|(key, value)| {
                if value == &checksum {
                    Some(key.to_path_buf())
                } else {
                    None
                }
            })
    }
}

impl Default for MyDirSync {
    fn default() -> Self {
        Self {
            path_to_checksum: RwLock::new(HashMap::new()),
            absolute_directory: PathBuf::new(),
        }
    }
}

#[tonic::async_trait]
impl DirSync for MyDirSync {
    async fn sync_structure(
        &self,
        request: Request<SyncRequest>,
    ) -> Result<Response<SyncResponse>, Status> {
        // Create key-value pairs for all elements in SyncRequest.get_elements()
        let mut path_set: HashSet<PathBuf> = HashSet::new();

        // Iterate over all elements
        for element in request.into_inner().elements.iter() {
            // Get path out of element, sanitize path and join with server's absolute directory
            let path = self
                .get_root_directory()
                .join(clean_path(element.path.clone()));
            path_set.insert(path.clone());

            // If element is a directory, create the directory
            if element.is_dir {
                println!("Creating directory: {:?}", path);
                create_dir_all(path)?;
            }
        }
        // Recursively walk over file tree from server's absolute directory (root is skipped!)
        walk_filetree_and_apply(self.get_root_directory(), &|entry| {
            if entry.path().as_path() != self.get_root_directory()
                && !path_set.contains(&entry.path())
            {
                // entry.path() not in client request -> delete on server
                println!("Removing file: {:?}", entry.path());
                if entry.path().is_dir() {
                    std::fs::remove_dir_all(entry.path())?;
                } else {
                    std::fs::remove_file(entry.path())?;
                }
                self.delete_checksum(entry.path());
            }
            Ok(())
        })?;

        Ok(Response::new(SyncResponse {}))
    }

    async fn diff_structure(
        &self,
        request: Request<DiffRequest>,
    ) -> Result<Response<DiffResponse>, Status> {
        // Iterate over all created elements from request, assemble path, create directory if it is
        // a directory
        let root_directory_path = self.get_root_directory();
        let consumed_request = request.into_inner();
        for element in consumed_request.created.iter() {
            if element.is_dir {
                let path = root_directory_path.join(clean_path(element.path.clone()));
                create_dir_all(path)?;
            }
        }

        // Iterate over all deleted elements from request, assemble path, remove directory if it is
        // a directory, delete checksum entry
        for element in consumed_request.deleted.iter() {
            let path = root_directory_path.join(clean_path(element.path.clone()));
            if element.is_dir {
                remove_dir_all(path)?;
            } else {
                remove_file(path)?;
            }
        }

        Ok(Response::new(DiffResponse {}))
    }

    async fn get_checksum(
        &self,
        request: Request<ChecksumRequest>,
    ) -> Result<Response<ChecksumResponse>, Status> {
        // Get path from request and create checksum for it (joined with root path)
        let checksum_request = request.into_inner();
        let path = self.absolute_directory.join(checksum_request.path);
        let block_size = checksum_request.block_size;
        let checksum = checksum_request.checksum;
        match compute_file_sha256(path.clone()) {
            Ok(Some(filehash)) => {
                // File exists and checksum was returned
                if filehash == checksum {
                    // Files match, nothing to do
                    return Ok(Response::new(ChecksumResponse {
                        path: path.to_string_lossy().into_owned(),
                        checksum,
                        checksums: vec![],
                    }));
                } else {
                    // Difference, file changed, try to copy
                    if let Some(found_path) = self.get_file_with_matching_checksum(checksum.clone())
                    {
                        match force_copy(found_path, path.clone()) {
                            Ok(_) => {
                                println!("Copied file from path {:?}", path.clone());
                                self.update_checksum(path.clone(), checksum.clone());
                                return Ok(Response::new(ChecksumResponse {
                                    path: path.to_string_lossy().into_owned(),
                                    checksum,
                                    checksums: vec![],
                                }));
                            }
                            Err(e) => {
                                // Return status with error
                                return Err(Status::aborted("Could not copy file"));
                            }
                        }
                    } else {
                        // no file with matching hash -> need client to stream blocks
                    }
                }
            }
            Ok(None) => {
                // File doesn't exist -> try to find copy
                if let Some(found_path) = self.get_file_with_matching_checksum(checksum.clone()) {
                    match force_copy(found_path, path.clone()) {
                        Ok(_) => {
                            println!("Copied file from path {:?}", path.clone());
                            self.update_checksum(path.clone(), checksum.clone());
                            return Ok(Response::new(ChecksumResponse {
                                path: path.to_str().unwrap().to_owned(),
                                checksum,
                                checksums: vec![],
                            }));
                        }
                        Err(e) => {
                            // Return status with error
                            return Err(Status::aborted("Could not copy file"));
                        }
                    }
                } else {
                    // no file with matching hash -> need client to stream blocks
                }
            }
            Err(e) => {
                // File exists, but could not be opened
                return Err(Status::aborted("Could not open file"));
            }
        }
        let file_chunker = FileChunker::new(path, block_size as usize);

        todo!("IMPLEMENT get_checksum()!");
    }

    async fn upload_blocks(
        &self,
        request: Request<Streaming<Block>>,
    ) -> Result<Response<UploadResponse>, Status> {
        todo!("IMPLEMENT upload_blocks()!");
    }
}
