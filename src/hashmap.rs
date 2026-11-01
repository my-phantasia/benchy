use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_hashmap(tsv_path: &str) -> Result<HashMap<String, u64>, Box<dyn std::error::Error>> {
    let file = File::open(tsv_path)?;
    let reader = BufReader::new(file);

    let mut map = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        if let Some((key, val_str)) = line.split_once('\t') {
            let val: u64 = val_str.trim().parse()?;
            map.insert(key.to_string(), val);
        }
    }

    Ok(map)
}
