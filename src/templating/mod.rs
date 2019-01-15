mod error;

use std::io::Read;
use std::fs;
use std::path::{Path,PathBuf};


#[derive(Deserialize)]
pub struct QuixFile {
    pub template: Metadata,
    pub variables: Vec<Variable>
}

impl QuixFile {
    pub fn open(path: &Path) -> Result<QuixFile, error::Error> {
        let mut path = PathBuf::from(path);
        if !path.ends_with("quix.json") {
            path.push("quix.json")
        }
        match fs::File::open(path) {
            Ok(mut file) => {
                let mut buf = String::new();
                if let Err(e) = file.read_to_string(&mut buf) {
                    return Err(error::Error::IOError(e))
                }
                serde_json::from_str(&buf).map_err(|e| error::Error::BadFormat(e))
            },
            Err(e) => Err(error::Error::IOError(e)),
        }
    }
}

#[derive(Deserialize)]
pub struct Metadata {
    pub name: String,
    pub author: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Variable {
    pub name: String,
    pub description: String,
    #[serde(rename = "default")]
    pub default_value: Option<String>,
}
