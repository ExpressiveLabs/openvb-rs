use std::path::Path;

use textgridde_rs::textgrid::{TextGrid, Tier};
use anyhow::{bail, Result};

use crate::time::Timestamp;
use crate::tools::pitch::write_pitch;
use crate::utterance::{FileDescriptor, Utterance};

pub fn from_lab<P: AsRef<Path>>(audio_path: P, lab_path: Option<P>) -> Result<FileDescriptor> {
    let lab_path = if let Some(p) = lab_path {
        p.as_ref().to_path_buf()
    } else {
        let mut path = audio_path.as_ref().to_path_buf();
        path.set_extension("lab");
        path
    };

    if !lab_path.exists() {
        bail!(".lab file does not exist: {:?}", lab_path);
    }
    
    // Read lab file to string and iterate over it
    let data = std::fs::read_to_string(lab_path)?;
    let data = data.lines().collect::<Vec<&str>>();

    let mut utterances = vec![];
    let mut intervals = vec![];

    let mut previous = String::from("sil");
    for interval in data.iter() {
        let mut parts = interval.splitn(3, " ");
        let start = parts.next();
        let end = parts.next();
        let label = parts.next();

        if start.is_none() || end.is_none() || label.is_none() {
            continue;
        }

        let start = start.unwrap().parse::<u64>()?;
        let end = end.unwrap().parse::<u64>()?;
        let label = label.unwrap();

        let label = match label {
            "SP" => "sil",
            "AP" => "br",

            "r0" => "r",
            "d0" => "d",

            _ => label
        };
        
        intervals.push((start, end, label));
    }
    
    for (i, (start, end, label)) in intervals.iter().enumerate() {
        let mut utterance = Utterance::default();
        
        // Parse timestamps
        utterance.start = Timestamp::new(start / 10);
        utterance.end = Timestamp::new(end / 10);
        utterance.midpoint = (utterance.end - utterance.start) / 2.0 + utterance.start;

        // Parse annotation data
        utterance.prev = previous.clone();
        utterance.curr = label.to_string();
        
        utterance.next = if i < data.len() - 1 {
            intervals[i + 1].2.to_string()
        } else {
            String::from("sil")
        };

        let flags = 0u8;

        // Parse pitch and meta
        utterance.pitch = None;
        utterance.flags = flags;

        utterances.push(utterance);

        previous = label.to_string();
    }


    // Fill the pitch fields
    write_pitch(&audio_path, &mut utterances)?;


    // Create the file descriptor
    let file = FileDescriptor {
        path: audio_path.as_ref().to_path_buf(),
        aliases: vec![],
        pitch: None,
        analysis_files: None,
        language: None,
        labels: utterances,
        extras: None
    };

    Ok(file)
}