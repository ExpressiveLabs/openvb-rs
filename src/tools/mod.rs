enum Phoneset {
    XSampa,
    IPA,
    Arpa,
    CZampa
}

#[cfg(feature = "generator")]
pub mod pitch;

#[cfg(feature = "translate")]
pub mod ipa;

#[cfg(feature = "translate")]
pub mod arpa;

mod scrape;
