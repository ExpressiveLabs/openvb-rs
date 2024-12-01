use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use anyhow::Result;

use crate::{library::Library, parser::textgrid::from_textgrid, tools::ipa::FromIPA, Singer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourcePhoneset {
    Arpabet,
    IPA,
    XSampa
}
impl SourcePhoneset {
    pub fn to_string(&self) -> String {
        match self {
            SourcePhoneset::Arpabet => "arpabet".to_string(),
            SourcePhoneset::IPA => "ipa".to_string(),
            SourcePhoneset::XSampa => "xsampa".to_string()
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "arpabet" => SourcePhoneset::Arpabet,
            "ipa" => SourcePhoneset::IPA,
            "xsampa" => SourcePhoneset::XSampa,
            _ => panic!("Invalid phoneset")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceDataType {
    TextGrid,
    OtoIni,
    Label
}
impl SourceDataType {
    pub fn to_string(&self) -> String {
        match self {
            SourceDataType::TextGrid => "textgrid".to_string(),
            SourceDataType::OtoIni => "otoini".to_string(),
            SourceDataType::Label => "label".to_string()
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "textgrid" => SourceDataType::TextGrid,
            "otoini" => SourceDataType::OtoIni,
            "label" => SourceDataType::Label,
            _ => panic!("Invalid data type")
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorConfig {
    pub language: String,
    pub name: String,
    pub phoneset: SourcePhoneset,
    pub data_type: SourceDataType,
    pub path: PathBuf
}

pub fn build(config: GeneratorConfig) -> Result<Singer> {
    let path = config.path;

    let mut singer = Singer::new();
    singer.meta.name = config.name;

    let mut lib = Library::default();
    lib.name = "Default".to_string();
    lib.is_default = true;

    // Get all files in directory
    let files = std::fs::read_dir(&path).unwrap();

    // Filter out non-wav files
    let files = files.filter(|f| {
        let file = f.as_ref().unwrap().path();
        file.extension().unwrap().to_str().unwrap() == "wav"
    }).map(|f| f.unwrap().path()).collect::<Vec<PathBuf>>();

    let pool = threadpool::ThreadPool::new(12);
    let (tx, rx) = std::sync::mpsc::channel();

    let files_len = files.len();

    // Iterate over files
    for file in tqdm::tqdm(files) {
        let tx = tx.clone();
        let data_type = config.data_type.clone();
        pool.execute(move || {
            let file = match data_type {
                SourceDataType::TextGrid => from_textgrid(&file, None),
                SourceDataType::OtoIni => {
                    unimplemented!()
                },
                SourceDataType::Label => {
                    unimplemented!()
                }
            };

            if file.is_err() {
                println!("Error: {:?}", file.err().unwrap());
                tx.send(None).unwrap();
                return;
            }

            tx.send(Some(file.unwrap())).unwrap();
        });
    }

    // Receive data
    for _ in tqdm::tqdm(0..files_len) {
        let file = rx.recv().unwrap();
        if let Some(file) = file {
            lib.files.push(file);
        }
    }

    // Register library
    singer.libraries.push(lib);

    // Convert phonemes if necessary
    match config.phoneset {
        SourcePhoneset::IPA => singer.from_ipa(),
        _ => {}
    }

    Ok(singer)
}

#[cfg(test)]
mod tests {
    use super::*;

    use dotenv::dotenv;

    #[test]
    fn test_from_nest() {
        dotenv().ok();

        let cfg = GeneratorConfig {
            path: PathBuf::from(std::env::var("TEXTGRID_TEST_DATASET").unwrap()),
            phoneset: SourcePhoneset::IPA,
            data_type: SourceDataType::TextGrid,
            name: String::from("Test Dataset"),
            language: String::from("en")
        };

        let singer = build(cfg.clone()).unwrap();

        let path = cfg.path.join("singer.json");
        singer.save(&path).unwrap();
    }
}