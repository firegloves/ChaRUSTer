use std::borrow::BorrowMut;
use std::fs::{File, read_to_string};
use std::io;
use std::io::BufRead;
use std::path::{Path, PathBuf};

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use toml::Value;

#[derive(Deserialize)]
pub struct Config {
    pub execution_conf: ExecutionConf,
    pub char_conf: CharacterConfig,
    pub values_conf: ValuesConfig,
}

#[derive(Deserialize)]
pub struct ExecutionConf {
    pub char_nums: u32
}

#[derive(Deserialize)]
pub struct CharacterConfig {
    pub gen_name: bool,
    pub gen_surname: bool,
    pub gen_nickname: bool,
    pub gen_birthdate: bool,
    pub gen_description: bool,
    pub gen_image: bool,
    pub gen_collection: bool,
    pub gen_profession: bool,
    pub gen_hobbies: bool,
    pub gen_props: bool,
    pub gen_levels: bool,
    pub gen_stats: bool,
}

#[derive(Deserialize)]
pub struct ValuesConfig {
    pub collection_name: String,
    pub names_file: String,
    pub surnames_file: String,
    pub nicknames_file: String,
    pub birthplaces_file: String,
    pub hobbies_file: String,
    pub professions_file: String,
    pub props_file: String,
    pub stats_file: String,
    pub levels_file: String,
    pub images_folder: String,
    pub description_files: String,
    pub birthdate_min_year: u16,
    pub birthdate_max_year: u16,
}

pub fn parse_config() -> Config {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/config.toml");
    let config_file = d.into_os_string().into_string().unwrap();
    toml::from_str(read_to_string(config_file).unwrap().as_str()).unwrap()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn should_read_conf_from_toml() {
        let config = parse_config();

        assert_eq!(config.execution_conf.char_nums, 3);

        assert!(config.char_conf.gen_name);
        assert!(config.char_conf.gen_surname);
        assert!(config.char_conf.gen_nickname);
        assert!(config.char_conf.gen_birthdate);
        assert!(config.char_conf.gen_description);
        assert!(config.char_conf.gen_image);
        assert!(config.char_conf.gen_collection);
        assert!(config.char_conf.gen_profession);
        assert!(config.char_conf.gen_hobbies);
        assert!(config.char_conf.gen_props);
        assert!(config.char_conf.gen_levels);
        assert!(config.char_conf.gen_stats);

        assert_eq!(config.values_conf.collection_name, "Junkie Zombies");
        assert_eq!(config.values_conf.names_file, "./names");
        assert_eq!(config.values_conf.surnames_file, "./surnames");
        assert_eq!(config.values_conf.nicknames_file, "./adjectives");
        assert_eq!(config.values_conf.hobbies_file, "./hobbies");
        assert_eq!(config.values_conf.professions_file, "./professions");
        assert_eq!(config.values_conf.props_file, "./properties");
        assert_eq!(config.values_conf.stats_file, "./stats");
        assert_eq!(config.values_conf.levels_file, "./levels");
        assert_eq!(config.values_conf.images_folder, "./images/");
        assert_eq!(config.values_conf.description_files, "./descriptions");
        assert_eq!(config.values_conf.birthdate_min_year, 1920);
        assert_eq!(config.values_conf.birthdate_max_year, 2010);
    }
}
