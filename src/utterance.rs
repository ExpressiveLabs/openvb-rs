use std::{collections::HashMap, path::PathBuf};

use intbits::Bits;
use serde::{Deserialize, Serialize};

use crate::time::Timestamp;

pub struct PhonemeFlags;
impl PhonemeFlags {
    pub const IS_VOWEL: u8 = 0b00000001;
}


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct FileDescriptor {
    pub path: PathBuf,
    pub aliases: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_files: Option<HashMap<String, PathBuf>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<HashMap<String, f32>>,
    
    pub labels: Vec<Utterance>
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Utterance {
    pub prev: String,
    pub curr: String,
    pub next: String,

    #[serde(skip)]
    pub audio_path: PathBuf,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch: Option<u8>,

    pub start: Timestamp,
    pub midpoint: Timestamp,
    pub end: Timestamp,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<HashMap<String, f32>>,

    pub flags: u8,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub areas: Option<HashMap<String, [Timestamp; 2]>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minified: Option<[u64; 3]>
}

impl Utterance {
    pub fn from_minified(data: &[u64; 3]) -> Self {
        let mut utterance = Utterance::default();
        utterance.minified = Some(*data);

        utterance.start = Timestamp::from(data[1]);
        utterance.end = Timestamp::from(data[2]);

        let prev = data[0].bits(0..16) as u16;
        let curr = data[0].bits(16..32) as u16;
        let next = data[0].bits(32..48) as u16;

        let flags = data[0].bits(48..56) as u8;
        let pitch = data[0].bits(57..64) as u8;

        utterance
    }
}