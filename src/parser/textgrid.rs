use std::path::Path;

use textgridde_rs::textgrid::{TextGrid, Tier};
use anyhow::{bail, Result};

use crate::time::Timestamp;
use crate::tools::pitch::write_pitch;
use crate::utterance::{FileDescriptor, Utterance};

pub fn from_textgrid<P: AsRef<Path>>(audio_path: P, textgrid_path: Option<P>) -> Result<FileDescriptor> {
    let textgrid_path = if let Some(p) = textgrid_path {
        p.as_ref().to_path_buf()
    } else {
        let mut path = audio_path.as_ref().to_path_buf();
        path.set_extension("TextGrid");
        path
    };

    if !textgrid_path.exists() {
        bail!("TextGrid file does not exist: {:?}", textgrid_path);
    }

    let tg = TextGrid::try_from(textgrid_path)?;

    let mut utterances = vec![];

    // Find a phones tier
    let mut data = None;
    for tier in tg.tiers() {
        match tier {
            Tier::IntervalTier(d) => {
                if d.name().eq(&"phones") {
                    data = Some(d.intervals().clone());
                    break;
                }
            },
            _ => {}
        }
    }

    if data.is_none() {
        bail!("Could not find phones tier in TextGrid file");
    }
    let data = data.unwrap();


    // Iterate over the tier's intervals
    let mut previous = String::from("sil");
    for (i, interval) in data.iter().enumerate() {
        let mut utterance = Utterance::default();

        // Parse timestamps
        utterance.start = Timestamp::from_seconds(*interval.xmin());
        utterance.end = Timestamp::from_seconds(*interval.xmax());

        // Parse annotation data
        utterance.prev = previous.clone();
        utterance.curr = interval.text().to_string();
        utterance.next = if i < data.len() - 1 {
            data[i + 1].text().to_string()
        } else {
            String::from("sil")
        };

        let flags = 0u8;

        // Parse pitch and meta
        utterance.pitch = None;
        utterance.flags = flags;

        utterances.push(utterance);

        previous = interval.text().to_string();
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
    };

    Ok(file)
}