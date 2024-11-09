pub mod singer;
pub mod utterance;
pub mod encode;
pub mod time;
pub mod tools;

#[cfg(feature = "generator")]
pub mod parser;

#[cfg(feature = "generator")]
pub mod generate;

pub use singer::Singer;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use singer::Singer;

    use super::*;

    #[test]
    fn test_load_singer_json() {
        let path = PathBuf::from("C:\\Users\\danie\\mikoto\\singer\\yamine_renri_vcv\\singer.json");
        let singer = Singer::load(&path).unwrap();

        println!("Loaded: {:#?}", singer.meta);
        println!("Singer has {} libraries:", singer.libraries.len());

        for library in singer.libraries {
            println!("\tLibrary: {} ({} files)", library.name, library.files.len());
        }
    }
}