use crate::case::{check_case, to_kebab};
use colored::Colorize;
use std::{
    env::args,
    fs::{self, DirEntry},
    path::Path,
};
mod case;

fn main() {
    let arguments: Vec<_> = args().collect();

    let mut target_dir = Path::new(".");

    if arguments.len() > 1 {
        target_dir = Path::new(&arguments[1]);
    }
    println!("Working on the following dir: {:?}", target_dir);

    let reads = read_directory(target_dir);
    let items_in_dir: Vec<CustomDirEntry> = reads.iter().map(CustomDirEntry::new).collect();

    for item in &items_in_dir {
        item.pretty_print();
    }

    for i in &items_in_dir {
        i.rename();
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
    new_name: Option<String>,
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
            new_name: None,
            kebab_name: to_kebab(&name, &current_case),
        }
    }

    fn rename(&self) {
        match self.new_name {
            None => match &self.kebab_name {
                None => println!("New name could be constructed automatically"),
                Some(x) => {
                    println!("Renaming {:?} => {:?}", self.item.path(), self.kebab_name);

                    let full_name = self.item.path().into_string().unwrap();
                    let new_full_name = full_name.replace(&self.name, x.as_str());
                    let renamed = fs::rename(full_name, new_full_name);
                    match renamed {
                        Err(x) => println!("Error renaming: {x}"),
                        Ok(_) => println!("We did it!"),
                    }
                }
            },
            Some(ref x) => {
                println!("Renaming {:?} => {:?}", self.item.path(), self.new_name);
                let full_name = self.item.path().into_string().unwrap();
                let new_full_name = full_name.replace(&self.name, x);
                let renamed = fs::rename(full_name, new_full_name);
                match renamed {
                    Err(x) => println!("Error renaming: {x}"),
                    Ok(_) => println!("We did it!"),
                }
            }
        }
    }

    fn pretty_print(&self) {
        let name = format!("{:?}", self.item.file_name());
        let new_name = format!("{:?}", self.kebab_name);
        match self.case {
            Case::Kebab => println!("{} => {}", name.green(), new_name),
            Case::Snake => println!("{} => {}", name.yellow(), new_name),
            Case::Camel => println!("{} => {}", name.cyan(), new_name),
            Case::Pascal => println!("{} => {}", name.blue(), new_name),
            Case::None => println!("{} => {}", name.red(), new_name),
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
