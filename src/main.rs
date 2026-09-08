use crate::{config_builder::Configs, custom_dir_entry::CustomDirEntry};
use colored::Colorize;
use serde::Deserialize;
use std::{
    env::args,
    fs::{self, DirEntry},
    io::{self, Write},
    path::Path,
};
mod case;
mod config_builder;
mod custom_dir_entry;

fn main() {
    let arguments: Vec<_> = args().collect();

    let mut target_dir = Path::new(".");

    if arguments.len() > 1 {
        target_dir = Path::new(&arguments[1]);
    }
    let configs = Configs::search_configs(target_dir);

    println!("{:?}", configs);
    println!();

    parse_dir(target_dir, &configs);
}

fn parse_dir(target_dir: &Path, configs: &Configs) {
    let reads = read_directory(target_dir);
    let items_in_dir: Vec<CustomDirEntry> = reads
        .iter()
        .map(|f| CustomDirEntry::new(f, configs))
        .collect();
    let directories: Vec<_> = items_in_dir.iter().filter(|f| f.is_dir).collect();
    let files: Vec<_> = items_in_dir.iter().filter(|f| !f.is_dir).collect();

    if !directories.is_empty() && configs.recursive_mode {
        for item in &directories {
            if !(configs.ignore_dotfiles && item.is_dotfile) {
                parse_dir(&item.item_data.path(), configs);
            } else {
                let message = format!("Skipping {} (dotfile)", item.name).green();
                println!("{message}");
            }
        }
    }

    println!(
        "\nWorking on the following dir: {}",
        target_dir.to_str().unwrap()
    );

    if !reads.is_empty() {
        for dir in &directories {
            print!("+ ");
            dir.pretty_print();
        }
        for file in &files {
            print!("- ");
            file.pretty_print();
        }
    }

    let count_to_rename: usize = items_in_dir
        .iter()
        .filter(|f| f.marked_to_rename)
        .collect::<Vec<_>>()
        .len();

    if count_to_rename > 0 {
        println!("\n{} items on: {:?}", items_in_dir.len(), target_dir);
        let confirmation = ask_confirmation(count_to_rename);
        if confirmation {
            // Renaming dirs
            for dir in &directories {
                dir.rename();
            }
            // Renaming files
            for file in &files {
                file.rename();
            }
        } else {
            println!("No changes where made!");
        }
    } else {
        println!("Nothing to do here");
    }
}

fn read_directory(path: &Path) -> Vec<DirEntry> {
    if path.parent().is_none() {
        panic!("Working on the root dir is not allowed!");
    }
    let message = format!("Failed to read the directory: {}", path.to_str().unwrap());
    let read = fs::read_dir(path).expect(&message);
    read.into_iter().map(|f| f.unwrap()).collect()
}

fn ask_confirmation(count: usize) -> bool {
    print!("Automatic rename is possible for {count} files/directories. Rename? y/[n]: ");
    let _ = io::stdout().flush();
    let mut user_confirmation = String::new();
    io::stdin()
        .read_line(&mut user_confirmation)
        .expect("Invalid response!");
    user_confirmation = user_confirmation.to_lowercase().trim().to_string();
    user_confirmation == "y" || user_confirmation == "yes"
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
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
    // This function is marked as a test but it's actual purpose is to create dummy files/dirs to work with
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
