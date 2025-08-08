

// use crate::{encode::PhonemeEncoder, library::Library, utterance::{FileDescriptor, Utterance}, Singer};

// pub trait FromARPA {
//     fn from_arpa(&mut self);
// }

// pub trait ToARPA {
//     fn to_arpa(&mut self);
// }

// impl FromARPA for Singer {
//     fn from_arpa(&mut self) {
//         for library in self.libraries.iter_mut() {
//             library.from_arpa();
//         }
//     }
// }

// impl FromARPA for Library {
//     fn from_arpa(&mut self) {
//         for file in self.files.iter_mut() {
//             file.from_arpa();
//         }
//     }
// }

// impl FromARPA for FileDescriptor {
//     fn from_arpa(&mut self) {
//         for label in self.labels.iter_mut() {
//             label.from_arpa();
//         }
//     }
// }

// impl FromARPA for Utterance {
//     fn from_arpa(&mut self) {
//         if !PhonemeEncoder::is_silence(&self.prev) {
//             self.prev = ipa_to_xsampa(&self.prev);
//         }

//         self.curr = ipa_to_xsampa(&self.curr);

//         if !PhonemeEncoder::is_silence(&self.next) {
//             self.next = ipa_to_xsampa(&self.next);
//         }
//     }
// }

// impl ToARPA for Singer {
//     fn to_arpa(&mut self) {
//         for library in self.libraries.iter_mut() {
//             library.to_arpa();
//         }
//     }
// }

// impl ToARPA for Library {
//     fn to_arpa(&mut self) {
//         for file in self.files.iter_mut() {
//             file.to_arpa();
//         }
//     }
// }

// impl ToARPA for FileDescriptor {
//     fn to_arpa(&mut self) {
//         for label in self.labels.iter_mut() {
//             label.to_arpa();
//         }
//     }
// }

// impl ToARPA for Utterance {
//     fn to_arpa(&mut self) {
//         if !PhonemeEncoder::is_silence(&self.prev) {
//             self.prev = xsampa_to_arpa(&self.prev);
//         }

//         self.curr = xsampa_to_arpa(&self.curr);

//         if !PhonemeEncoder::is_silence(&self.next) {
//             self.next = xsampa_to_arpa(&self.next);
//         }
//     }
// }
