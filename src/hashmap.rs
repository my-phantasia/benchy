use ahash::AHashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_hashmap(tsv_path: &str) -> Result<AHashMap<String, u64>, Box<dyn std::error::Error>> {
    let file = File::open(tsv_path)?;
    let mut reader = BufReader::new(file);

    let mut map = AHashMap::with_capacity(4_665_811);
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
