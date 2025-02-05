use color_eyre::Result;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{fs, path::PathBuf};

use super::Repository;

#[derive(Debug, Default)]
pub struct AptSource {
    pub _enabled: bool,
    pub uri: String,
}

#[derive(Debug, Default)]
pub struct AptList {
    pub file_path: PathBuf,
    pub sources: Vec<AptSource>,
}

const APT_SOURCES_LIST_D_PATH: &str = "/etc/apt/sources.list.d";
const _APT_SOURCES_LIST_PATH: &str = "/etc/apt/sources.list";

fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
pub struct AptRepositories {
    pub initialized: bool,
    pub items: Vec<AptList>,
}

impl Default for AptRepositories {
    fn default() -> Self {
        Self {
            initialized: false,
            items: Vec::default(),
        }
    }
}

impl AptRepositories {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Repository for AptRepositories {
    fn check_for_repository(&self) -> bool {
        let apt_d_dir = fs::read_dir(APT_SOURCES_LIST_D_PATH);
        match apt_d_dir {
            Ok(apt_list) => !apt_list.count() > 0,
            Err(_) => false,
        }
    }

    fn load_repository_list(&mut self) -> Result<i32> {
        let apt_list_d = fs::read_dir(APT_SOURCES_LIST_D_PATH)?;
        // TODO: include sources.list
        let regex = Regex::new("^\\s{0,}#{0,}\\s{0,}deb").unwrap();
        let mut total = 0;
        for path in apt_list_d {
            let mut apt_list = AptList::default();
            let path = path?.path();
            apt_list.file_path = path.clone();
            if let Ok(lines) = read_lines(&path) {
                let filtered_lines = lines.filter(|x| match x {
                    Ok(line) => regex.is_match(line),
                    Err(_) => false,
                });

                for line in filtered_lines {
                    let line_str = line?;
                    apt_list.sources.push(AptSource {
                        uri: line_str.clone(), // going to eat the small perf cost here
                        _enabled: !line_str.starts_with("#"),
                    });
                    total += 1;
                }

                self.items.push(apt_list);
            }
        }
        self.initialized = true;
        Ok(total)
    }

    fn get_repository_list(&self) -> Vec<String> {
        self.items
            .iter()
            .flat_map(|apt_list| apt_list.sources.iter().map(|source| source.uri.clone()))
            .collect()
    }
}
