use std::path::Path;

use anyhow::{Result, bail};
use rsworld::{dio, stonemask};
use rsworld_sys::DioOption;
use wavers::{Samples, Wav};


use crate::utterance::Utterance;

pub fn detect_pitch_from_samples(data: &Vec<f64>, sr: i32) -> f64 {
    let dio_option = DioOption {
        f0_floor: 71.0,
        f0_ceil: 1760.0,
        frame_period: 5.0,
        channels_in_octave: 2.0,
        speed: 1,
        allowed_range: 0.1,
    };

    let (t, rough_f0) = dio(data, sr, &dio_option);
    let f0 = stonemask(data, sr, &t, &rough_f0);
    let f0_avg = f0.iter().sum::<f64>() / f0.len() as f64;

    f0_avg
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

        let start = utterance.start.samples(sample_rate).max(0);
        let end = utterance.end.samples(sample_rate).clamp(0, samples.len());

        assert!(start < end, "Start ({}) must be less than end ({}) (in {:?} @ {})", start, end, file, utterance.curr);
        // println!("Path: {:?}, Start: {}, End: {}, label: {}", file, start, end, utterance.curr);

        let pitch = detect_pitch_from_samples(&samples[start..end].to_vec(), wav.sample_rate());
        let midi = ftom(pitch as f32);

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
