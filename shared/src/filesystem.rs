use anyhow::{Context, anyhow};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, DirEntry, File, copy},
    io::{self, Error, Read, prelude::*},
    path::{Path, PathBuf},
};

pub fn force_copy(source: PathBuf, target: PathBuf) -> Result<(), Error> {
    copy(source, target)?;
    Ok(())
}

pub fn clean_path(path: String) -> PathBuf {
    Path::new(&path).canonicalize().unwrap()
}

pub fn walk_filetree_and_apply<F>(dir: &Path, callback: &F) -> std::io::Result<()>
where
    F: Fn(&DirEntry) -> std::io::Result<()>,
{
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_filetree_and_apply(&path, callback)?;
            }
            callback(&entry)?;
        }
    }
    Ok(())
}

pub fn compute_file_sha256(path: PathBuf) -> Result<Option<String>, anyhow::Error> {
    if Path::exists(path.as_path()) {
        let mut file = File::open(path).context("Failed to open file")?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .context("Failed to read file content into string")?;
        let hash = Sha256::digest(content)
            .iter()
            .map(|x| x.to_owned())
            .collect::<Vec<u8>>();
        let result = String::from_utf8(hash).context("Failed to build string from UTF-8 hash")?;
        Ok(Some(result))
    } else {
        Ok(None)
    }
}

pub fn compute_bytes_sha256(bytes: Vec<u8>) -> String {
    let hash = Sha256::digest(bytes)
        .iter()
        .map(|x| x.to_owned())
        .collect::<Vec<u8>>();
    String::from_utf8(hash).unwrap()
}

// FileChunker turns file into chunks to compute the checksums per block
// If two files differ, try to sync only differing blocks
pub struct FileChunker {
    path: PathBuf,
    block_size: usize,
    file: std::fs::File,
}

impl FileChunker {
    pub fn new(path: PathBuf, block_size: usize) -> Result<Self, anyhow::Error> {
        if block_size == 0 {
            return Err(anyhow!("Block size cannot be zero!"));
        }
        let file = File::open(path.clone()).context("Failed to open file")?;
        Ok(Self {
            path,
            block_size,
            file,
        })
    }
}

impl Iterator for FileChunker {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = vec![0u8; self.block_size];
        match self.file.read(&mut buffer) {
            Ok(0) => None, // EOF,
            Ok(n) => {
                buffer.truncate(n);
                Some(buffer)
            }
            Err(e) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_iterator() {
        // Create a file with serial content
        let write_buffer = "Hello, world!".as_bytes();
        let test_file_path = "sample.txt";
        let mut f = File::create(test_file_path).unwrap();
        f.write_all(write_buffer).unwrap();

        // Read content
        let chunker = FileChunker::new(PathBuf::from(test_file_path), 2).unwrap();
        let read_buffer = chunker.flatten().collect::<Vec<u8>>();

        // Check that written and read buffers match
        assert_eq!(write_buffer, read_buffer);

        // Clean up file
        std::fs::remove_file(test_file_path).unwrap();
    }
}
