use std::error::Error;
use std::path::Path;

#[derive(Debug)]
pub struct ExtType {
    pub ext: String,
    pub count: u32,
    pub bytes: u64,
}

impl ExtType {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let p = path.as_ref();
        let metadata = p.metadata()?;

        let ext = p
            .extension()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("")
            .to_string();

        Ok(ExtType {
            ext,
            count: 1,
            bytes: metadata.len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_txt_file() {
        let et = ExtType::from_path("test.txt");
        println!("DEBUG: {et:?}");
    }
}
