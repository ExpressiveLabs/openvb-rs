// To keep file sizes small, OpenVB can encode phoneme data into a binary format. This file provides structs to deal with encoding dictionaries.
// Phonemes are stored as 16-bit unsigned integers, but we only use the first 10 bits. That should cover all phonemes in the entire universe (jk)


use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

// Phonemes are stored in X-SAMPA format
const PHONEME_DATA: [&str; 135] = ["sil", "br", "pau", "cl", "<RES>", "<RES>", "<RES>", "<RES>", "a", "b", "b_<", "c", "d", "d`", "d_<", "e", "f", "g", "g_<", "h", "h\\", "i", "j", "j\\", "k", "l", "l`", "l\\", "m", "n", "n`", "o", "p", "p\\", "q", "r", "r`", "r\\", "r\\`", "s", "s`", "s\\", "t", "t`", "u", "v", "v\\", "w", "x", "x\\", "y", "z", "z`", "z\\", "A", "B", "B\\", "C", "D", "E", "F", "G", "G\\", "G\\_<", "H", "H\\", "I", "I\\", "J", "J\\", "J\\_<", "K", "K\\", "L", "L\\", "M", "M\\", "N", "N\\", "O", "O\\", "P", "Q", "R", "R\\", "S", "T", "U", "U\\", "V", "W", "X", "X\\", "Y", "Z", ".", "\"", "%", "'", ":", ":\\", "-", "@", "@\\", "@`", "{", "}", "1", "2", "3", "3\\", "4", "5", "6", "7", "8", "9", "&", "?", "?\\", "*", "/", "<", "<\\", ">", ">\\", "^", "!", "!\\", "|", "|\\", "||", "|\\|\\", "=\\", "-\\"];

const DIACRITICS_DATA: [&str; 49] = ["_\"", "_+", "_-", "_/", "_0", "_<", "=", "_>", "_?", "_\\", "_^", "_}", "`", "~", "_~", "_A", "_a", "_B", "_B_L", "_c", "_d", "_e", "<F>", "_F", "_G", "_H", "_H_T", "_h", "_j", "_k", "_L", "_l", "_M", "_m", "_N", "_n", "_O", "_o", "_q", "<R>", "_R", "_R_F", "_r", "_T", "_t", "_v", "_w", "_X", "_x"
];

const SILENCE: [&str; 4] = ["sil", "pau", "br", "cl"];

// Data layout:
//     Everything left-padded
//
//     Prev/curr/next phoneme: 16 bits each
//     Phoneme (10 bits) + Diacritic (6 bits)
//     Config data: 8 bits
//     - is_vowel
//     - ???
//     Pitch: 7+1 bits
//  
//     LABEL:
//     [16]|[16]|[16]|[8]|[7] -> PREV|CURR|NEXT|FLAGS|PITCH
//
//     [64]|[64]|[64] -> LABEL|START|END

pub struct PhonemeEncoder;

impl PhonemeEncoder {
    pub fn encode(data: &str) -> u16 {
        let mut phoneme = 0;
        
        // Split data into phoneme and diacritic
        let mut phoneme_data = data;
        let mut diacritic_data = "";

        if data.contains("_") {
            let split: Vec<&str> = data.split("_").collect();
            phoneme_data = split[0];
            diacritic_data = split[1];
        }

        for (i, p) in PHONEME_DATA.iter().enumerate() {
            if p == &phoneme_data {
                phoneme = i as u16;
                break;
            }
        }

        for (i, d) in DIACRITICS_DATA.iter().enumerate() {
            if d == &diacritic_data {
                phoneme |= (i as u16) << 10;
                break;
            }
        }

        phoneme
    }

    pub fn decode(data: u16) -> String {
        let phoneme = data & 0b0000000000111111;
        let diacritic = (data & 0b1111111111000000) >> 10;

        // Print bits
        println!("Phoneme: {:b}", phoneme);
        println!("Diacritic: {:b}", diacritic);

        let mut phoneme_data = PHONEME_DATA[phoneme as usize].to_string();
        if diacritic != 0 {
            phoneme_data.push_str("_");
            phoneme_data.push_str(DIACRITICS_DATA[diacritic as usize]);
        }

        phoneme_data
    }

    pub fn is_silence<S: AsRef<str>>(data: S) -> bool {
        let data = data.as_ref();
        SILENCE.contains(&data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let data = "G_/";

        let encoded = PhonemeEncoder::encode(data);
        let decoded = PhonemeEncoder::decode(encoded);

        println!("Encoded: {:b}", encoded);
        println!("Decoded: {}", decoded);

        assert_eq!(data, decoded);
    }
}