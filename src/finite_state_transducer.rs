use fst::{Map, MapBuilder};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter};

pub fn bake_fst(tsv_path: &str, fst_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(tsv_path)?;
    let reader = BufReader::new(file);

    let wtr = BufWriter::new(File::create(fst_path)?);
    let mut builder = MapBuilder::new(wtr)?;

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        if let Some((key, val_str)) = line.split_once('\t') {
            let val: u64 = val_str.trim().parse()?;
            builder.insert(key, val)?;
        }
    }

    builder.finish()?;
    Ok(())
}

pub fn load_fst(fst_path: &str) -> Result<Map<Vec<u8>>, Box<dyn std::error::Error>> {
    let bytes = fs::read(fst_path)?;
    let map = Map::new(bytes)?;
    Ok(map)
}
