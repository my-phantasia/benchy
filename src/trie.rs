use fast_radix_trie::StringRadixMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_trie(tsv_path: &str) -> Result<StringRadixMap<u64>, Box<dyn std::error::Error>> {
    let file = File::open(tsv_path)?;
    let mut reader = BufReader::new(file);

    let mut map = StringRadixMap::new();
    let mut buffer = String::new();

    while reader.read_line(&mut buffer)? > 0 {
        let trimmed = buffer.trim();

        if !trimmed.is_empty() {
            if let Some((key, val_str)) = trimmed.split_once('\t') {
                let val: u64 = val_str.trim().parse()?;
                map.insert(key.to_string(), val);
            }
        }

        buffer.clear();
    }

    Ok(map)
}
