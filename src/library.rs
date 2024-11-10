use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{singer::Language, utterance::FileDescriptor};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    pub name: String,
    pub uuid: Uuid,
    pub base_path: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    pub is_default: bool,
    pub files: Vec<FileDescriptor>
}

impl Iterator for Library {
    type Item = FileDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        self.files.pop()
    }
}