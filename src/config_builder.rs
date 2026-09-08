use crate::Case;
use serde::Deserialize;
use std::{
    fs,
    path::{self, Path},
};

#[derive(Debug, Deserialize)]
pub struct Configs {
    pub case: Case,
    pub ignore_dotfiles: bool,
    // pub ignore_gitignore: bool,
    pub recursive_mode: bool,
    // pub ignore_pattern: Vec<String>,
    // pub min_name_length: u8,
    // pub max_name_length: u8,
    // pub no_confirm: bool,
}

#[derive(Debug, Deserialize)]
struct OptionalConfigs {
    pub case: Option<String>,
    pub ignore_dotfiles: Option<bool>,
    // pub ignore_gitignore: Option<bool>,
    pub recursive_mode: Option<bool>,
    // pub ignore_pattern: Option<Vec<String>>,
    // pub min_name_length: Option<u8>,
    // pub max_name_length: Option<u8>,
    // pub no_confirm: Option<bool>,
}
impl Configs {
    pub fn get_defaults() -> Configs {
        Configs {
            case: Case::Kebab,
            ignore_dotfiles: true,
            // ignore_gitignore: false,
            recursive_mode: true,
            // ignore_pattern: vec![],
            // min_name_length: 5,
            // max_name_length: 40,
            // no_confirm: false,
        }
    }

    pub fn search_configs(target: &Path) -> Configs {
        let name = String::from("bireme.toml");
        let target = path::absolute(target).unwrap();
        let mut target = target.as_path();

        let mut full_path = Path::join(target, &name);
        let mut exist = fs::exists(&full_path).unwrap();

        while !exist {
            if !target.parent().is_none() {
                target = target.parent().unwrap();
                full_path = Path::join(target, &name);
                exist = fs::exists(&full_path).unwrap();
            } else {
                break;
            }
        }
        let mut defaults = Self::get_defaults();

        if exist {
            println!("Using custom configs!");
            let content = fs::read_to_string(full_path).unwrap();
            let new_configs: OptionalConfigs = toml::from_str(&content).unwrap();

            if let Some(x) = new_configs.case {
                match x.as_str() {
                    "kebab" => defaults.case = Case::Kebab,
                    "snake" => defaults.case = Case::Snake,
                    _ => panic!("Error reading configurations: {x} is not a valid option"),
                }
            }
            if let Some(x) = new_configs.ignore_dotfiles {
                defaults.ignore_dotfiles = x;
            }
            // if let Some(x) = new_configs.ignore_gitignore {
            //     defaults.ignore_gitignore = x;
            // }
            if let Some(x) = new_configs.recursive_mode {
                defaults.recursive_mode = x;
            }
            // if let Some(x) = new_configs.ignore_pattern {
            //     defaults.ignore_pattern = x;
            // }
            // if let Some(x) = new_configs.min_name_length {
            //     defaults.min_name_length = x;
            // }
            // if let Some(x) = new_configs.max_name_length {
            //     defaults.max_name_length = x;
            // }
            // if let Some(x) = new_configs.no_confirm {
            //     defaults.no_confirm = x;
            // }
        }
        defaults
    }
}
