#![allow(dead_code)]
use anyhow::Result;
use std::io::{BufRead, BufReader, Write};

// TODO
// - - - file - - -
// ✅ check file exist
// ✅ write (append) to new file, fail if file exists
// ✅ read existing file
// get file metadata (permissions, owner, size, etc.)
// append lines to existing text file
// - - - directory - - -
// check dir exist
// list dir
// create new dir
// remove existing dir
// - - - utils - - -
// implement cat
// implement wc
// implement dir traversal
// implement find
// implement cp
// implement mv

fn file_exists(path: &str) -> anyhow::Result<bool> {
    let exists = std::fs::exists(path)?;
    if exists {
        println!("File {} exists!", &path);
    } else {
        println!("File {} not found!", &path);
    }
    Ok(exists)
}

fn write_to_new_file(name: &str, content: &str) -> anyhow::Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(name)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}

fn read_file(path: &str) -> Result<()> {
    let file_content = std::fs::read_to_string(path)?;
    println!("File content:\n============\n{}\n==========", &file_content);
    Ok(())
}

fn read_file_lines_by_line() -> Result<()> {
    // --- Read file line by line ---
    let file = std::fs::File::open("example.txt")?;
    let reader = BufReader::new(file);

    println!("File contents:");
    for line_result in reader.lines() {
        let line = line_result?;
        println!("{}", line);
    }

    Ok(())
}

fn main() {
    let this_path = std::path::Path::new(file!());
    let file_name = this_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("<file name w/o rs>");
    println!("==========================================================");
    println!("Run tests:\ncargo t --example {}", &file_name);
    println!("==========================================================");
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_write_to_new_file_success() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        let content = "first line\nsecond line";

        let result = write_to_new_file(file_path.to_str().unwrap(), content);

        assert!(
            result.is_ok(),
            "Failed to create and write file: {:?}",
            result
        );

        // Verify content
        let written_content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(written_content, content, "Content mismatch");

        // temp_dir automatically cleans up
    }

    #[test]
    fn test_write_to_new_file_fails_if_exists() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("duplicate.txt");
        let content = "test content";

        // First write should succeed
        write_to_new_file(file_path.to_str().unwrap(), content).unwrap();

        // Second write should fail
        let result = write_to_new_file(file_path.to_str().unwrap(), content);
        assert!(
            result.is_err(),
            "Expected error when file exists, got: {:?}",
            result
        );
    }

    #[test]
    fn test_write_empty_content() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("empty.txt");

        let result = write_to_new_file(file_path.to_str().unwrap(), "");

        assert!(result.is_ok());
        let written_content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(written_content, "");
    }

    #[test]
    fn test_file_exists() {
        let f = tempfile::NamedTempFile::new().unwrap();
        let exists = file_exists(f.path().to_str().unwrap());
        assert!(exists.is_ok());
        assert!(exists.unwrap());
    }

    #[test]
    fn test_file_exists_not() {
        let exists = file_exists("fake.file");
        assert!(exists.is_ok());
        assert!(!exists.unwrap());
    }
}
