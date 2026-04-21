use std::{fs, path::Path};

use anyhow::{Error, anyhow};

pub fn copy_dir_recursive(source: &Path, destination: &Path) -> Result<(), Error> {
    let source_metadata = fs::metadata(source)?;
    if !source_metadata.is_dir() {
        return Err(anyhow!("Source '{}' is not a directory", source.display()));
    }

    // Create destination directory (and parents)
    fs::create_dir_all(destination)?;

    // Iterate over entries in source
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let entry_path = entry.path();
        let entry_type = entry.file_type()?;

        // Compute relative path from source to entry
        let relative_path = entry_path.strip_prefix(source)?;
        let target_path = destination.join(relative_path);

        if entry_type.is_file() {
            // Copy file
            fs::copy(&entry_path, &target_path)?;
            println!("Copied file: {}", entry_path.display());
        } else if entry_type.is_dir() {
            // Recursively copy directory
            copy_dir_recursive(&entry_path, &target_path)?;
            println!("Copied directory: {}", entry_path.display());
        }
    }

    Ok(())
}
