use std::{
    env::args,
    fs::{self, DirEntry},
    path::Path,
};

fn main() {
    println!("Hello");
    let arguments: Vec<_> = args().collect();

    let mut target_dir = Path::new(".");

    if arguments.len() > 1 {
        target_dir = Path::new(&arguments[1]);
    }
    println!("Working on the following dir: {:?}", &target_dir);

    let reads = read_directory(target_dir);
    let mut items_in_dir: Vec<CustomDirEntry> =
        reads.iter().map(|f| CustomDirEntry::new(f, None)).collect();

    let mut iter = 0;
    for i in &mut items_in_dir {
        let name = format!("new name {:?}", iter);
        i.assign_name(name);
        iter = iter + 1;
    }

    for i in &items_in_dir {
        i.rename();
    }
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
}

impl<'a> CustomDirEntry<'a> {
    fn new(entry: &'a DirEntry, new_name: Option<String>) -> CustomDirEntry<'a> {
        let name = entry.file_name().into_string().unwrap();
        CustomDirEntry {
            item: entry,
            name: name,
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
    fn assign_name(&mut self, another_name: String) {
        self.new_name = Some(another_name);
    }
}
