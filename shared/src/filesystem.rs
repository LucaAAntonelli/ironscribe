use std::{fs::copy, io::Error, path::Path};

pub fn force_copy(source: String, target: String) -> Result<(), Error> {
    copy(source, target)?;
    Ok(())
}

pub fn clean_path(path: String) -> String {
    if path == "" {
        return "".to_string();
    }
    Path::new(&path).canonicalize().unwrap().to_str().unwrap().to_owned()
}