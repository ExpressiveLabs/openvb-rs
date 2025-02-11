#[cfg(test)]
mod tests {
    use crate::Singer;

    #[test]
    fn test_scrape_symbols() {
        let path = "/media/rosa/Rosa 1TB/dataset/ritsu_v2/phrases/singer.json";
        let singer = Singer::load(path).unwrap();

        let mut symbols = vec![];
        for itm in singer.libraries[0].files.iter() {
            for label in itm.labels.iter() {
                if !symbols.contains(&label.curr) {
                    symbols.push(label.curr.clone());
                }
            }
        }

        println!("{:?}", symbols);
    }
}
