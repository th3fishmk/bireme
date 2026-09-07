use crate::case::{check_case, to_kebab};
use colored::Colorize;
use std::{
    env::args,
    fs::{self, DirEntry},
    io::{self, Write},
    path::Path,
};
mod case;

fn main() {
    let arguments: Vec<_> = args().collect();

    let mut target_dir = Path::new(".");

    if arguments.len() > 1 {
        target_dir = Path::new(&arguments[1]);
    }
    println!("\nWorking on the following dir: {:?}\n", target_dir);

    let reads = read_directory(target_dir);
    let items_in_dir: Vec<CustomDirEntry> = reads.iter().map(CustomDirEntry::new).collect();

    for item in &items_in_dir {
        item.pretty_print();
    }

    println!(
        "\nFound a total of: {} files/directories",
        items_in_dir.len()
    );

    let count: usize = items_in_dir
        .iter()
        .filter(|f| f.case != Case::Kebab)
        .collect::<Vec<_>>()
        .len();

    if count > 0 {
        print!(
            "Automatic rename is possible for {count} files/directories.\nDo you want to continue: y/[n]: "
        );
        let _ = io::stdout().flush();
        let mut user_confirmation = String::new();
        io::stdin()
            .read_line(&mut user_confirmation)
            .expect("Invalid response!");
        user_confirmation = user_confirmation.to_lowercase().trim().to_string();
        if user_confirmation == "y" || user_confirmation == "yes" {
            println!("Renaming...");
            for i in &items_in_dir {
                i.rename();
            }
        } else {
            println!("See you soon! (Threat)")
        }
    } else {
        println!("Nothing to do!")
    }
}

fn read_directory(path: &Path) -> Vec<DirEntry> {
    let read = fs::read_dir(path).unwrap();
    read.into_iter().map(|f| f.unwrap()).collect()
}

#[derive(Debug)]
struct CustomDirEntry<'a> {
    item: &'a DirEntry,
    name: String,
    kebab_name: Option<String>,
    case: Case,
}

impl<'a> CustomDirEntry<'a> {
    fn new(entry: &'a DirEntry) -> CustomDirEntry<'a> {
        let name = entry.file_name().into_string().unwrap();
        let current_case = check_case(name.as_str());
        CustomDirEntry {
            item: entry,
            name: name.clone(),
            case: current_case.clone(),
            kebab_name: if current_case == Case::Kebab {
                None
            } else {
                to_kebab(&name)
            },
        }
    }

    fn rename(&self) {
        if self.case != Case::Kebab {
            match &self.kebab_name {
                None => println!("New name could NOT be constructed automatically"),
                Some(new_name) => {
                    print!("Renaming {:?} => {:?}\r", self.item.path(), new_name);
                    let old_full_name = self.item.path().into_string().unwrap();
                    let old_name = &self.name;
                    let mut new_full_name = old_full_name.trim_end_matches(old_name).to_string();
                    new_full_name = Path::join(Path::new(&new_full_name), new_name)
                        .into_string()
                        .unwrap();

                    let file_exist = fs::exists(&new_full_name);
                    match file_exist {
                        Err(x) => {
                            let message = "The following error has occur:".red();
                            print!("{message} {x}");
                        }
                        Ok(x) => {
                            if x {
                                let message = "ERROR: ".red();
                                let context = format!(
                                    "A file/directory with the name {new_full_name} already exist!"
                                );
                                print!("{message} {context}");
                            } else {
                                let renamed = fs::rename(&old_full_name, &new_full_name);
                                match renamed {
                                    Err(_) => {
                                        let message = "Error:".red();
                                        print!("{message} {} x {}", old_full_name, new_full_name);
                                    }
                                    Ok(_) => {
                                        let message = "Renamed:".green();
                                        print!("{message} {} => {}", old_full_name, new_full_name);
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

    fn pretty_print(&self) {
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

#[derive(Debug, PartialEq, Clone)]
pub enum Case {
    Kebab,
    Snake,
    Camel,
    Pascal,
    None,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, path::PathBuf};
    fn getter_dirnames() -> Vec<&'static str> {
        let dummy_dirs = vec![
            "dummy-dir",
            "dummy_dir",
            "dummy_ directory",
            "dummy dir",
            "dummYd🥶ir",
            "dummdir",
        ];
        dummy_dirs
    }
    fn getter_filenames() -> Vec<&'static str> {
        let dummy_files = vec![
            "dummy-file.txt",
            "dummyfile..c",
            "dummyFile.jpg",
            "dummy_file.bak",
            "dummy. file",
            "dummyfile01.file.",
            "Dummy-file.mp3",
            "DummyFile.exe",
        ];
        dummy_files
    }
    #[test]
    fn create_temporal_files_and_dirs() {
        let path = PathBuf::from("dummy");
        let dummy_dirs = getter_dirnames();
        let dummy_files = getter_filenames();

        for dir_name in dummy_dirs {
            let dir_path = &path.join(dir_name);
            let _ = fs::create_dir_all(dir_path).expect("Failed to create directory");
        }
        for dummy in &dummy_files {
            let file_path = &path.join(dummy);
            let _ = File::create(file_path);
        }
    }
}
