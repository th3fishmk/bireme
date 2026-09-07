use std::{
    fs::{self, DirEntry},
    path::Path,
};

use crate::{
    Case,
    case::{check_case, to_kebab},
};
use colored::Colorize;

#[derive(Debug)]
pub struct CustomDirEntry<'a> {
    pub item: &'a DirEntry,
    pub name: String,
    pub kebab_name: Option<String>,
    pub case: Case,
    pub is_dir: bool,
}

impl<'a> CustomDirEntry<'a> {
    pub fn new(entry: &'a DirEntry) -> CustomDirEntry<'a> {
        let name = entry.file_name().into_string().unwrap();
        let current_case = check_case(name.as_str());
        // let is_dir = ;
        CustomDirEntry {
            item: entry,
            name: name.clone(),
            case: current_case.clone(),
            kebab_name: if current_case == Case::Kebab {
                None
            } else {
                to_kebab(&name)
            },
            is_dir: entry.file_type().unwrap().is_dir(),
        }
    }

    pub fn rename(&self) {
        if self.case != Case::Kebab {
            match &self.kebab_name {
                None => println!("New name could NOT be constructed automatically"),
                Some(new_name) => {
                    let old_full_name = self.item.path().into_string().unwrap();
                    let old_name = &self.name;
                    let mut new_full_name = old_full_name.trim_end_matches(old_name).to_string();
                    new_full_name = Path::join(Path::new(&new_full_name), new_name)
                        .into_string()
                        .unwrap();

                    print!("Renaming {} => {:?}\r", old_name, new_name);
                    let file_exist = fs::exists(&new_full_name);
                    match file_exist {
                        Err(x) => {
                            let message = "The following error has occur:".red();
                            print!("{message} {x}\r");
                        }
                        Ok(x) => {
                            if x {
                                let message = "ERROR: ".red();
                                let context = format!(
                                    "A file/directory with the name {new_full_name} already exist!"
                                );
                                print!("{message} {context}\r");
                            } else {
                                let renamed = fs::rename(&old_full_name, &new_full_name);
                                match renamed {
                                    Err(_) => {
                                        let message = "Error:".red();
                                        print!("{message} {} x {}\r", old_full_name, new_full_name);
                                    }
                                    Ok(_) => {
                                        let message = "Renamed:".green();
                                        print!("{message} {} => {}\r", old_name, new_name);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            println!();
        }
    }

    pub fn pretty_print(&self) {
        let name = format!("{:?}", self.item.file_name());
        let new_name = format!("{:?}", self.kebab_name.clone().unwrap_or("".to_string()));
        match self.case {
            Case::Kebab => {
                if self.kebab_name.is_some() {
                    println!("{} => {}", name.green(), new_name)
                } else {
                    println!("{}", name.green())
                }
            }
            Case::Snake => {
                if self.kebab_name.is_some() {
                    println!("{} => {}", name.cyan(), new_name)
                } else {
                    println!("{}", name.cyan())
                }
            }
            Case::Camel => {
                if self.kebab_name.is_some() {
                    println!("{} => {}", name.cyan(), new_name)
                } else {
                    println!("{}", name.cyan())
                }
            }
            Case::Pascal => {
                if self.kebab_name.is_some() {
                    println!("{} => {}", name.yellow(), new_name)
                } else {
                    println!("{}", name.yellow())
                }
            }
            Case::None => {
                if self.kebab_name.is_some() {
                    println!("{} => {}", name.red(), new_name)
                } else {
                    println!("{}", name.red())
                }
            }
        };
    }
}
