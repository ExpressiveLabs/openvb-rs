use ipa_translate::{ipa_to_xsampa, xsampa_to_ipa};

use crate::{encode::PhonemeEncoder, singer::SingerResourceBundle, utterance::{FileDescriptor, Utterance}, Singer};

pub trait FromIPA {
    fn from_ipa(&mut self);
}

pub trait ToIPA {
    fn to_ipa(&mut self);
}

impl FromIPA for Singer {
    fn from_ipa(&mut self) {
        for library in self.libraries.iter_mut() {
            library.from_ipa();
        }
    }
}

impl FromIPA for SingerResourceBundle {
    fn from_ipa(&mut self) {
        for file in self.files.iter_mut() {
            file.from_ipa();
        }
    }
}

impl FromIPA for FileDescriptor {
    fn from_ipa(&mut self) {
        for label in self.labels.iter_mut() {
            label.from_ipa();
        }
    }
}

impl FromIPA for Utterance {
    fn from_ipa(&mut self) {
        if !PhonemeEncoder::is_silence(&self.prev) {
            self.prev = ipa_to_xsampa(&self.prev);
        }
        
        self.curr = ipa_to_xsampa(&self.curr);

        if !PhonemeEncoder::is_silence(&self.next) {
            self.next = ipa_to_xsampa(&self.next);
        }
    }
}

impl ToIPA for Singer {
    fn to_ipa(&mut self) {
        for library in self.libraries.iter_mut() {
            library.to_ipa();
        }
    }
}

impl ToIPA for SingerResourceBundle {
    fn to_ipa(&mut self) {
        for file in self.files.iter_mut() {
            file.to_ipa();
        }
    }
}

impl ToIPA for FileDescriptor {
    fn to_ipa(&mut self) {
        for label in self.labels.iter_mut() {
            label.to_ipa();
        }
    }
}

impl ToIPA for Utterance {
    fn to_ipa(&mut self) {
        if !PhonemeEncoder::is_silence(&self.prev) {
            self.prev = xsampa_to_ipa(&self.prev);
        }
        
        self.curr = xsampa_to_ipa(&self.curr);

        if !PhonemeEncoder::is_silence(&self.next) {
            self.next = xsampa_to_ipa(&self.next);
        }
    }
}