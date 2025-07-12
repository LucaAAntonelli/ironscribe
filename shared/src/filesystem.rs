use sha2::{Digest, Sha256};
use std::{
    fs::{self, DirEntry, copy},
    io::Error,
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

pub fn compute_sha256(path: PathBuf) -> String {
    let hash = Sha256::digest(path.to_str().unwrap())
        .iter()
        .map(|x| x.to_owned())
        .collect::<Vec<u8>>();
    String::from_utf8(hash).unwrap()
}
