use std::{
    collections::{
        btree_map::Entry,
        BTreeMap,
    },
    str::FromStr,
};

use crate::{
    error::MCError,
    Result,
};

#[derive(Debug, Default)]
pub struct Stats {
    pub stats: BTreeMap<String, String>,
    pub slabs: BTreeMap<u8, BTreeMap<String, usize>>,
}

impl Stats {
    fn new() -> Self {
        Self::default()
    }

    /// Add memcached slab stats
    fn add_stat_slab(&mut self, slab: u8, stat: String, value: usize) -> Result<()> {
        if let Entry::Vacant(e) = self.slabs.entry(slab) {
            let mut inside = BTreeMap::new();
            inside.insert(stat, value);
            e.insert(inside);
        } else {
            self.slabs
                .get_mut(&slab)
                .ok_or_else(|| MCError::Stat(format!("add_stat_slab {} {} {}", slab, stat, value)))?
                .insert(stat, value);
        }
        Ok(())
    }

    /// Add general memcached stat
    fn add_stat(&mut self, stat: String, value: String) {
        self.stats.entry(stat).or_insert_with(|| value);
    }
}

impl FromStr for Stats {
    type Err = MCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split("\r\n").collect();

        let mut stats = Self::new();

        for line in lines {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let ["STAT", stat, value] = parts[..] {
                let parts: Vec<_> = stat.split(':').collect();
                if let ["items", slab, stat] = parts[..] {
                    stats.add_stat_slab(
                        slab.parse::<u8>()?,
                        stat.to_string(),
                        value.parse::<usize>()?,
                    )?
                } else if let [slab, stat] = parts[..] {
                    stats.add_stat_slab(
                        slab.parse::<u8>()?,
                        stat.to_string(),
                        value.parse::<usize>()?,
                    )?
                } else if let [stat] = parts[..] {
                    stats.add_stat(stat.to_string(), value.to_string());
                } else {
                    return Err(MCError::Parse);
                }
            }
        }
        Ok(stats)
    }
}
