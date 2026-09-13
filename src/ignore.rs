use std::{
    fs,
    path::{self, Path},
};

use globset::{Glob, GlobSet, GlobSetBuilder};

pub fn read_gitignore(target_dir: &Path) -> GlobSet {
    let target_dir = path::absolute(target_dir).unwrap();
    let name = ".gitignore";
    let full_path = Path::join(&target_dir, name);

    let mut global = GlobSetBuilder::new();

    let exist = fs::exists(&full_path).unwrap();
    if exist {
        println!("git found:");
        let content = fs::read_to_string(full_path).unwrap().trim().to_string();
        let content: Vec<_> = content.split("\n").collect();
        for glob in content {
            println!("adding {glob}");
            global.add(Glob::new(glob).unwrap());
        }
    }
    global.build().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_gitignore_pattern() {
        let target_dir = Path::new(".");
        let gitignore = read_gitignore(target_dir);
        let target_dir = gitignore.is_match("/target");
        let dummy_dir = gitignore.is_match("/dummy");
        assert!(target_dir);
        assert!(dummy_dir);
    }
}
