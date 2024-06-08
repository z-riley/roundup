use std::convert::From;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct FileSummary {
    pub name: String,
    pub extension: Option<String>,
}

impl From<String> for FileSummary {
    fn from(path: String) -> Self {
        FileSummary {
            name: path.clone(),
            extension: get_extension(&path),
        }
    }
}

impl FileSummary {
    pub fn read_num_lines(&self) -> Result<u64, Box<dyn Error>> {
        let file = File::open(&self.name).map_err(|e| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to open file: {}", e),
            )) as Box<dyn Error>
        })?;
        let reader = BufReader::new(file);
        let lines = reader.lines().count();

        Ok(lines as u64)
    }
}

fn get_extension(name: &str) -> Option<String> {
    if name.contains(".") {
        name.split(".").into_iter().last().map(|s| s.to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_extension() {
        assert_eq!(get_extension("myrust.rs").unwrap(), String::from("rs"));
        assert_eq!(get_extension("my.rust.rs").unwrap(), String::from("rs"));
        assert_eq!(get_extension("noextension"), None);
    }
}
