use std::path::Path;

use anyhow::{Result, bail};
use wavers::{Samples, Wav};
use yin::Yin;


use crate::utterance::Utterance;

pub fn detect_pitch_from_samples(samples: &[f64], sample_rate: f64) -> f32 {
    const FMAX: f64 = 800.0;
    
    let samples = if (samples.len() as f64) < FMAX {
        // Pad symmetrically with zeros
        let pad = (FMAX + 1.0 - samples.len() as f64).round() as usize;
        let mut padded = vec![0.0; pad];
        padded.extend_from_slice(samples);
        padded.extend_from_slice(&vec![0.0; pad]);

        padded
    } else {
        samples.to_vec()
    };

    // Get the pitch
    let estimator = Yin::init(0.05, 50.0, FMAX, sample_rate as usize);
    let pitch = estimator.estimate_freq(&samples);

    pitch as f32
}

pub fn write_pitch<P: AsRef<Path>>(file: P, config: &mut Vec<Utterance>) -> Result<()> {
    // Make sure that the file is supported (wav only)
    let file = file.as_ref();
    let ext = file.extension().unwrap_or_default();
    if ext != "wav" {
        bail!("Unsupported file extension: {} ({})", ext.to_string_lossy(), file.display());
    }

    // Read the file
    let mut wav: Wav<f32> = Wav::from_path(file).unwrap();
	let samples: Samples<f32> = wav.read().unwrap();
    let sample_rate = wav.sample_rate() as f64;

    let samples = samples.iter().map(|s| *s as f64).collect::<Vec<f64>>();

    // Get the pitch
    for utterance in config.iter_mut() {
        if utterance.curr.eq("sil") {
            continue;
        }

        let start = utterance.start.samples(sample_rate);
        let end = utterance.end.samples(sample_rate);

        // println!("Start: {}, End: {}, label: {}", start, end, utterance.curr);

        let pitch = detect_pitch_from_samples(&samples[start..end], sample_rate);
        let midi = ftom(pitch);

        if pitch.is_infinite() || pitch.is_nan() {
            // println!("Pitch is infinite or NaN");
            continue;
        }

        // println!("Pitch: {} / {}", pitch, midi);

        utterance.pitch = Some(midi);
    }

    *config = config.iter().filter(|u| u.pitch.is_some()).cloned().collect();

    Ok(())
}

pub fn mtof(midi: u8) -> f32 {
    440.0 * 2.0_f32.powf((midi as f32 - 69.0) / 12.0)
}

pub fn ftom(freq: f32) -> u8 {
    (69.0 + 12.0 * (freq / 440.0).log2()).round() as u8
}