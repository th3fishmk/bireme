use crate::{
    Case::{self},
    case::{check_case, kebab_to_others, to_kebab},
    config_builder::Configs,
};
use colored::{ColoredString, Colorize};
use std::{
    fs::{self, DirEntry},
    path::Path,
};

#[derive(Debug)]
pub struct CustomDirEntry<'a> {
    pub item_data: &'a DirEntry,
    pub current_case: Case,
    pub target_case: Case,
    pub is_dir: bool,
    pub is_dotfile: bool,
    pub marked_to_rename: bool,
    pub name: String,
    pub new_name: Option<String>,
}

impl<'a> CustomDirEntry<'a> {
    pub fn new(entry: &'a DirEntry, configs: &Configs) -> CustomDirEntry<'a> {
        let name = entry.file_name().into_string().unwrap();
        let current_case = check_case(&name);
        let dotfile = name.starts_with(".");
        let check_dotfile = if dotfile {
            !configs.ignore_dotfiles
        } else {
            true
        };

        let case_change_req = current_case != configs.case;
        let all_true = case_change_req && check_dotfile;

        let kebab_name = to_kebab(&name, &current_case);
        let new_cased_name: Option<String>;
        match kebab_name {
            Some(x) => new_cased_name = kebab_to_others(&x, &configs.case),
            None => new_cased_name = None,
        };
        CustomDirEntry {
            item_data: entry,
            name: name.clone(),
            current_case: current_case,
            target_case: configs.case.clone(),
            is_dir: entry.file_type().unwrap().is_dir(),
            is_dotfile: dotfile,
            marked_to_rename: all_true,
            new_name: new_cased_name,
        }
    }

    pub fn rename(&self) {
        if self.marked_to_rename {
            match &self.new_name {
                None => println!("New name could NOT be constructed automatically"),
                Some(new_name) => {
                    let old_full_name = self.item_data.path().into_string().unwrap();
                    let old_name = &self.name;
                    let mut new_full_name = old_full_name.trim_end_matches(old_name).to_string();
                    new_full_name = Path::join(Path::new(&new_full_name), &new_name)
                        .into_string()
                        .unwrap();

                    print!("Renaming {} => {}\r", old_name, &new_name);
                    let file_exist = fs::exists(&new_full_name);
                    match file_exist {
                        Err(x) => {
                            let message = "The following error has occur:".red();
                            println!("{message} {x}\r");
                        }
                        Ok(x) => {
                            if x {
                                let message = "ERROR: ".red();
                                let context = format!(
                                    "A file/directory with the name {new_full_name} already exist!"
                                );
                                println!("{message} {context}\r");
                            } else {
                                let renamed = fs::rename(&old_full_name, &new_full_name);
                                match renamed {
                                    Err(_) => {
                                        let message = "Error:".red();
                                        println!(
                                            "{message} {} x {}\r",
                                            old_full_name, new_full_name
                                        );
                                    }
                                    Ok(_) => {
                                        let message = "Renamed:".green();
                                        println!("{message} {} => {}\r", old_name, new_name);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn pretty_print(&self) {
        let mut name: ColoredString = self.name.red();
        match self.current_case {
            Case::Kebab => name = name.green(),
            Case::Snake => name = name.yellow(),
            Case::Camel => name = name.cyan(),
            Case::Pascal => name = name.yellow(),
            Case::None => name = name.red(),
        };
        if self.marked_to_rename {
            let new_full_name = &self.new_name;
            match new_full_name {
                None => {
                    println!("We couldn't get you a name for: {name}")
                }
                Some(new_name) => {
                    let mut new_name = new_name.red();
                    match self.target_case {
                        Case::Kebab => new_name = new_name.green(),
                        Case::Snake => new_name = new_name.yellow(),
                        Case::Camel => new_name = new_name.cyan(),
                        Case::Pascal => new_name = new_name.yellow(),
                        Case::None => new_name = new_name.red(),
                    };
                    println!("{name} => {new_name}");
                }
            }
        } else {
            println!("{name}");
        }
    }
}
