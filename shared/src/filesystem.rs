use adler32::RollingAdler32;
use anyhow::{Context, anyhow};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, DirEntry, File, copy},
    io::{Error, Read},
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

pub struct Hasher {}

impl Hasher {
    pub fn compute_file_hash(&self, path: PathBuf) -> Result<Option<[u8; 32]>, anyhow::Error> {
        // One-shot hash, i.e. compute hash and consume directly
        if Path::exists(path.as_path()) {
            let mut file = File::open(path).context("Failed to open file")?;
            let mut content = String::new();
            file.read_to_string(&mut content)
                .context("Failed to read file content into string")?;
            let result = Sha256::digest(content).into();
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }

    pub fn compute_weak_byte_hash(&self, bytes: &[u8]) -> u32 {
        let mut adler32 = RollingAdler32::new();
        adler32.update_buffer(bytes);
        adler32.hash()
    }

    pub fn compute_strong_byte_hash(&self, bytes: &[u8]) -> [u8; 32] {
        let mut sha256 = Sha256::new();
        sha256.update(bytes);
        sha256.finalize().into()
    }
}

// FileChunker turns file into chunks to compute the checksums per block
// If two files differ, try to sync only differing blocks
pub struct FileChunker {
    block_size: usize,
    file: std::fs::File,
}

impl FileChunker {
    pub fn new(path: PathBuf, block_size: usize) -> Result<Self, anyhow::Error> {
        if block_size == 0 {
            return Err(anyhow!("Block size cannot be zero!"));
        }
        let file = File::open(path.clone()).context("Failed to open file")?;
        Ok(Self { block_size, file })
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
            Err(e) => {
                eprintln!("Error when iterating over {:?}: {}", self.file, e);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
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
