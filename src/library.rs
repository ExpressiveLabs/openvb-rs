use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{singer::Language, time::Timestamp, utterance::{FileDescriptor, Utterance}};

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

impl Library {
    pub fn iter_labels(&self) -> impl Iterator<Item = &Utterance> {
        self.files.iter().flat_map(|f| f.labels.iter())
    }
}

impl Iterator for Library {
    type Item = FileDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        self.files.pop()
    }
}