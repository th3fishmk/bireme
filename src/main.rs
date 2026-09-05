use crate::case::check_case;
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
    println!("Working on the following dir: {:?}", &target_dir);

    let reads = read_directory(target_dir);
    let items_in_dir: Vec<CustomDirEntry> =
        reads.iter().map(|f| CustomDirEntry::new(f, None)).collect();

    for item in &items_in_dir {
        item.pretty_print();
    }

    // Actual renaming
    // for i in &items_in_dir {
    //     i.rename();
    // }

    // let mut iter = 0;
    // for i in &mut items_in_dir {
    //     let name = format!("new name {:?}", iter);
    //     i.assign_name(name);
    //     iter = iter + 1;
    // }
}

fn read_directory(path: &Path) -> Vec<DirEntry> {
    let read = fs::read_dir(path).unwrap();
    let results = read.into_iter().map(|f| f.unwrap()).collect();
    results
}

#[derive(Debug)]
struct CustomDirEntry<'a> {
    item: &'a DirEntry,
    name: String,
    new_name: Option<String>,
    case: Case,
}

impl<'a> CustomDirEntry<'a> {
    fn new(entry: &'a DirEntry, new_name: Option<String>) -> CustomDirEntry<'a> {
        let name = entry.file_name().into_string().unwrap();
        CustomDirEntry {
            item: entry,
            name: name.clone(),
            case: check_case(name.as_str()),
            new_name: new_name,
        }
    }

    fn rename(&self) {
        match self.new_name {
            None => {
                println!("No new name!")
            }
            Some(ref x) => {
                println!("Renaming {:?} => {:?} ||", self.item.path(), self.new_name);
                let full_name = self.item.path().into_string().unwrap();
                let new_full_name = full_name.replace(&self.name, &x);
                let renamed = fs::rename(full_name, new_full_name);
                println!("");
                match renamed {
                    Err(x) => println!("Error renaming: {x}"),
                    Ok(_) => println!("We did it!"),
                }
            }
        }
    }

    fn pretty_print(&self) {
        let print = format!("{:?}", self.item.file_name());
        match self.case {
            Case::Kebab => println!("{}", print.green()),
            Case::Snake => println!("{}", print.yellow()),
            Case::Camel => println!("{}", print.cyan()),
            Case::Pascal => println!("{}", print.blue()),
            Case::None => println!("{}", print.red()),
        };
    }
}

#[derive(Debug, PartialEq)]
pub enum Case {
    Kebab,
    Snake,
    Camel,
    Pascal,
    None,
}
