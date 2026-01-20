use std::fmt::Display;
use std::fs;
use std::path::Path;

use anyhow::Result;
use log::info;
use serde::Deserialize;

pub enum Mode {
    Words,
    Numbers,
}

pub struct TotemConfig {
    pub mode: Mode,
    pub totems: Vec<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum TotemList {
    Numbers(Vec<u32>),
    Words(Vec<String>),
}

impl Display for TotemList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TotemList::Numbers(n) => write!(f, "numbers count={}", n.len()),
            TotemList::Words(w) => write!(f, "words count={}", w.len()),
        }
    }
}

impl TotemConfig {
    pub fn load<P>(config_file: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let file_data = fs::read_to_string(config_file.as_ref())?;
        let list: TotemList = serde_json::from_str(&file_data)?;

        info!("totem: {list}");

        let (mode, totems) = match list {
            TotemList::Numbers(nums) => {
                let totems = nums.into_iter().map(|n| format!("{:06}", n)).collect();
                (Mode::Numbers, totems)
            }
            TotemList::Words(words) => (Mode::Words, words),
        };

        Ok(Self { mode, totems })
    }
}
