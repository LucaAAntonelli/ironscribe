use sha2::{Digest, Sha256};
use std::{
    fs::{self, DirEntry, File, copy},
    io::{Error, prelude::*},
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

pub fn compute_file_sha256(path: PathBuf) -> Result<String, anyhow::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let hash = Sha256::digest(content)
        .iter()
        .map(|x| x.to_owned())
        .collect::<Vec<u8>>();
    let result = String::from_utf8(hash)?;
    Ok(result)
}
