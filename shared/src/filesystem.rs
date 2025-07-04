use std::{io::Error, fs::copy};

pub fn force_copy(source: String, target: String) -> Result<(), Error> {
    copy(source, target)?;
    Ok(())
}
