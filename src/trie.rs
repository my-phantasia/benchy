use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default, Debug)]
pub struct TrieNode {
    pub val: Option<u64>,
    pub children: HashMap<char, TrieNode>,
}

#[derive(Default, Debug)]
pub struct Trie {
    pub root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, key: &str, value: u64) {
        let mut current = &mut self.root;
        for ch in key.chars() {
            current = current.children.entry(ch).or_default();
        }
        current.val = Some(value);
    }

    pub fn get(&self, key: &str) -> Option<u64> {
        let mut current = &self.root;
        for ch in key.chars() {
            current = current.children.get(&ch)?;
        }
        current.val
    }
}

pub fn load_trie(tsv_path: &str) -> Result<Trie, Box<dyn std::error::Error>> {
    let file = File::open(tsv_path)?;
    let reader = BufReader::new(file);

    let mut trie = Trie::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        if let Some((key, val_str)) = line.split_once('\t') {
            let val: u64 = val_str.trim().parse()?;
            trie.insert(key, val);
        }
    }

    Ok(trie)
}
