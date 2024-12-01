use std::{path::PathBuf, sync::Arc};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use anyhow::Result;

use crate::{library::Library, utterance::FileDescriptor};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Singer {
    pub meta: Meta,
    pub origin: Origin,
    pub language: Language,
    pub libraries: Vec<Library>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_fields: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_fields: Option<Vec<(String, String)>>
}

impl Singer {
    #[cfg(feature = "generator")]
    pub fn new() -> Self {
        Singer {
            meta: Meta::default(),
            origin: Origin::now(),
            language: Language::default(),
            libraries: vec![],

            ..Default::default()
        }
    }

    pub fn load(path: &PathBuf) -> Result<Self> {
        // Get file extension
        let ext = path.extension().unwrap().to_str().unwrap();

        // Branch based on file extension: json or bin
        match ext {
            "json" => Self::load_json(path),
            "bin" => Self::load_bin(path),
            _ => Err(anyhow::anyhow!("Unsupported file extension: {}", ext))
        }
    }

    fn load_bin(path: &PathBuf) -> Result<Self> {
        let data = std::fs::read(path)?;
        Ok(bincode::deserialize(&data)?)
    }

    fn load_json(path: &PathBuf) -> Result<Self> {
        let data = std::fs::read_to_string(path)?;
        let mut res: Self = serde_json::from_str(&data)?;

        let base_path = path.parent().unwrap().to_path_buf();

        for lib in res.libraries.iter_mut() {
            for file in lib.files.iter_mut() {
                file.labels.sort_by(|a, b| a.start.value.cmp(&b.start.value));

                for label in file.labels.iter_mut() {
                    label.audio_path = base_path.join(&file.path);
                }
            }
        }

        Ok(res)
    }

    pub fn save(&self, path: &PathBuf) -> Result<()> {
        // Get file extension
        let ext = path.extension().unwrap().to_str().unwrap();

        // Branch based on file extension: json or bin
        match ext {
            "json" => self.save_json(path),
            "bin" => self.save_bin(path),
            _ => Err(anyhow::anyhow!("Unsupported file extension: {}", ext))
        }
    }

    fn save_bin(&self, path: &PathBuf) -> Result<()> {
        let data = bincode::serialize(&self)?;
        std::fs::write(path, data)?;
        Ok(())
    }

    fn save_json(&self, path: &PathBuf) -> Result<()> {
        let data = serde_json::to_string_pretty(&self)?;
        std::fs::write(path, data)?;
        Ok(())
    }

    pub fn get_default(&self) -> Option<&Library> {
        let r0 = self.libraries.iter().find(|lib| lib.is_default);

        r0.or(self.libraries.first())
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub name: String,
    pub uuid: Uuid,
    pub icon: PathBuf
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Origin {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Author>,
    pub application: String,
    pub creation_date: String
}

impl Origin {
    #[cfg(feature = "generator")]
    pub fn now() -> Self {
        Origin {
            application: String::from("OpenVBgen"),
            creation_date: chrono::Utc::now().to_rfc3339(),
            ..Default::default()
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,
    pub url: String
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    pub default: String,
    pub supported: Vec<String>
}