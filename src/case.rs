use crate::Case::{self};
use diacritics;
use regex::Regex;

pub fn check_case(target: &str) -> Case {
    if check_for_kebab_case(target) {
        Case::Kebab
    } else if check_for_snake_case(target) {
        Case::Snake
    } else if check_for_camel_case(target) {
        Case::Camel
    } else if check_for_pascal_case(target) {
        Case::Pascal
    } else {
        Case::None
    }
}

fn check_for_kebab_case(target: &str) -> bool {
    let kebab_regex = Regex::new(r"^[_.]?([a-z0-9]+-)*[a-z0-9]+(\.[a-z0-9\-]+)*$").unwrap();
    kebab_regex.is_match(target)
}
fn check_for_snake_case(target: &str) -> bool {
    let kebab_regex = Regex::new(r"^[_.]?([a-z0-9]+_)*[a-z0-9]+(\.[a-z0-9\_]+)*$").unwrap();
    kebab_regex.is_match(target)
}
fn check_for_camel_case(target: &str) -> bool {
    let kebab_regex = Regex::new(r"^[_.]?[a-z][a-z0-9]*([A-Z][a-z0-9]*)*(\.[a-z0-9]+)*$").unwrap();
    kebab_regex.is_match(target)
}
fn check_for_pascal_case(target: &str) -> bool {
    let kebab_regex = Regex::new(r"^[_.]?([A-Z][a-z0-9]*)+(\.[a-z0-9]+)*$").unwrap();
    kebab_regex.is_match(target)
}

pub fn from_kebab(target: &str, target_case: &Case) -> Option<String> {
    match target_case {
        Case::Kebab => {
            // println!("Positive for kebab!");
            Some(target.to_string())
        }
        Case::Snake => {
            // println!("Snaking!");
            Some("snaky_file".to_string())
        }
        _ => {
            println!("Negative for _");
            None
        }
    }
}

pub fn to_kebab(target: &str, current_case: &Case) -> Option<String> {
    let invalid = Regex::new(r"[^a-zA-Z0-9\.]").unwrap();
    let double_allowed = Regex::new(r"\-{2,}|\_{2,}|\.{2,}").unwrap();
    // let double_period = Regex::new(r"\.{2,}").unwrap();
    let spaces = Regex::new(r"\s").unwrap();
    let mut target = diacritics::remove_diacritics(target);
    let leading_period = target.starts_with(".");
    if leading_period {
        target = target.trim_start_matches(".").to_string();
    }
    let leading_underscore = target.starts_with("_");
    if leading_underscore {
        target = target.trim_start_matches("-").to_string();
    }
    target = invalid.replace_all(&target, " ").to_string();
    target = double_allowed.replace_all(&target, " ").to_string();
    target = target.trim().to_string();
    target = spaces.replace_all(&target, "-").to_string();
    let mut brand_new_name: String;
    match current_case {
        Case::Kebab => brand_new_name = target.clone(),
        Case::Snake => brand_new_name = snake_to_kebab(&target),
        Case::Camel => brand_new_name = camel_to_kebab(&target),
        Case::Pascal => brand_new_name = pascal_to_kebab(&target),
        Case::None => brand_new_name = none_to_kebab(&target),
    };
    if check_case(&brand_new_name) == Case::Kebab {
        println!("before leading: {}", brand_new_name);
        if leading_period {
            brand_new_name.insert(0, '.');
        } else if leading_underscore {
            brand_new_name.insert(0, '_');
        }
        // brand_new_name = double_period.replace_all(&brand_new_name, ".").to_string();
        // println!("Positive for {:?}", &target);
        println!("after leading: {}", brand_new_name);
        Some(brand_new_name)
    } else {
        // println!("Negative for {:?}", &target);
        None
    }
}

fn camel_to_kebab(target: &str) -> String {
    // eprintln!("We shouldn't be here");
    target.to_string()
}
fn pascal_to_kebab(target: &str) -> String {
    // eprintln!("We shouldn't be here");
    target.to_string()
}
fn none_to_kebab(target: &str) -> String {
    let double_allowed = Regex::new(r"\-{2,}").unwrap();
    let parts: Vec<_> = target.split(".").collect();
    let mut processed = vec![];
    for part in parts {
        let mut pice = part.to_lowercase();
        pice = pice.trim().to_string();
        pice = pice.replace(" ", "-");
        pice = pice.trim_start_matches("-").to_string();
        pice = pice.trim_end_matches("-").to_string();
        processed.push(pice);
    }
    let mut result = processed.join(".");
    result = result.trim_end_matches(".").to_string();
    result = double_allowed.replace_all(&result, "-").to_string();
    result
}
fn snake_to_kebab(target: &str) -> String {
    let double_allowed = Regex::new(r"\-{2,}").unwrap();
    let parts: Vec<_> = target.split(".").collect();
    let mut processed = vec![];
    for part in parts {
        let mut pice = part.to_lowercase();
        pice = pice.trim().to_string();
        pice = pice.replace(" ", "_");
        pice = pice.trim_start_matches("_").to_string();
        pice = pice.trim_end_matches("_").to_string();
        processed.push(pice);
    }
    let mut result = processed.join(".");
    result = result.trim_end_matches(".").to_string();
    result = double_allowed.replace_all(&result, "-").to_string();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kebab_names() -> Vec<&'static str> {
        let names = vec![
            "main.js",
            ".main.js",
            "app-config.json",
            "user-profile.png",
            "get-sorted-data.ts",
            "item-123-count.txt",
            "v1-api-routes.go",
            "_main.js",
            "_app-config.json",
            "_user-profile.png",
            "_get-sorted-data.ts",
            "_item-123-count.txt",
            "_archive.tar.gz",
            "projects",
            ".projects",
            "app-config",
            "user-profile",
            "get-sorted-data",
            "item-123-count",
            "v1-api-routes",
            "_projects",
            "_app-config",
            "_user-profile",
            "_get-sorted-data",
            "_item-123-count",
            "_archive-data",
        ];
        names
    }
    fn snake_names() -> Vec<&'static str> {
        let names = vec![
            // "main.js", // this defaults to kebab
            // ".main.js", // this defaults to kebab
            "app_config.json",
            "user_profile.png",
            "get_sorted_data.ts",
            "item_123_count.txt",
            "v1_api_routes.go",
            // "_main.js", // this defaults to kebab
            "_app_config.json",
            "_user_profile.png",
            "_get_sorted_data.ts",
            "_item_123_count.txt",
            // "_archive.tar.gz", // this defaults to kebab
            // "projects", // this defaults to kebab
            // ".projects", // this defaults to kebab
            "app_config",
            "user_profile",
            "get_sorted_data",
            "item_123_count",
            "v1_api_routes",
            // "_projects", // this defaults to kebab
            "_app_config",
            "_user_profile",
            "_get_sorted_data",
            "_item_123_count",
            "_archive_data",
        ];
        names
    }
    fn kebab_ed_snake() -> Vec<&'static str> {
        let names = vec![
            // "main.js", // this defaults to kebab
            // ".main.js", // this defaults to kebab
            "app-config.json",
            "user-profile.png",
            "get-sorted-data.ts",
            "item-123-count.txt",
            "v1-api-routes.go",
            // "_main.js", // this defaults to kebab
            "_app-config.json",
            "_user-profile.png",
            "_get-sorted-data.ts",
            "_item-123-count.txt",
            // "_archive.tar.gz", // this defaults to kebab
            // "projects", // this defaults to kebab
            // ".projects", // this defaults to kebab
            "app-config",
            "user-profile",
            "get-sorted-data",
            "item-123-count",
            "v1-api-routes",
            // "_projects", // this defaults to kebab
            "_app-config",
            "_user-profile",
            "_get-sorted-data",
            "_item-123-count",
            "_archive-data",
        ];
        names
    }
    fn camel_names() -> Vec<&'static str> {
        let names = vec![
            // "main.js", // this defaults to kebab
            // ".main.js", // this defaults to kebab
            "appConfig.json",
            "userProfile.png",
            "getSortedData.ts",
            "item123Count.txt",
            "v1ApiRoutes.go",
            // "_main.js", // this defaults to kebab
            "_appConfig.json",
            "_userProfile.png",
            "_getSortedData.ts",
            "_item123Count.txt",
            // "_archive.tar.gz", // this defaults to kebab
            // "projects", // this defaults to kebab
            // ".projects", // this defaults to kebab
            "appConfig",
            "userProfile",
            "getSortedData",
            "item123Count",
            "v1ApiRoutes",
            // "_projects", // this defaults to kebab
            "_appConfig",
            "_userProfile",
            "_getSortedData",
            "_item123Count",
            "_archiveData",
        ];
        names
    }
    fn pascal_names() -> Vec<&'static str> {
        let names = vec![
            "Main.js",
            ".Main.js",
            "AppConfig.json",
            "UserProfile.png",
            "GetSortedData.ts",
            "Item123Count.txt",
            "V1ApiRoutes.go",
            "_Main.js",
            "_AppConfig.json",
            "_UserProfile.png",
            "_GetSortedData.ts",
            "_Item123Count.txt",
            "_Archive.tar.gz",
            "Projects",
            ".Projects",
            "AppConfig",
            "UserProfile",
            "GetSortedData",
            "Item123Count",
            "V1ApiRoutes",
            "_Projects",
            "_AppConfig",
            "_UserProfile",
            "_GetSortedData",
            "_Item123Count",
            "_ArchiveData",
        ];
        names
    }
    fn no_convention_names() -> Vec<&'static str> {
        let names = vec![
            "..New Folder",
            "._New Folder",
            ".New Folder",
            "biology_notes_unit3_DO_NOT_DELETE.pdf",
            "Budget 2026 $$$ (FIXED).xlsx",
            "Class Notes Fall 2025",
            "cool music project [WIP]",
            "cool song edit final_v3_really_final.mp3",
            "Desktop Clean Up (Old)",
            "DO NOT DELETE",
            "Draft 1 final (1).docx",
            "essay_NEW_v2_FINAL.pdf",
            "family photo - summer vacation (1).jpeg",
            "IMG_4092.JPG",
            "IMPORTANT - READ THIS",
            "lab report - chemistry (backup).docx",
            "math homework ch1-4",
            "math homework chapter 4.pages",
            "misc stuff",
            "my resume 2026.pdf",
            "New Folder (2)",
            "New Folder - Copy",
            "New Folder",
            "Photos (1)",
            "Presentation_for_class_FINAL2.pptx",
            "Project Notes & Idea List.txt",
            "Project Stuff & Drafts",
            "receipts & invoices",
            "recipe - grandma's cookies.txt",
            // "screen shot 2026-03-12 at 4.15.22 PM.png",
            "stuff for website.zip",
            "Tax Documents 2025 $$$",
            "Taxes_2025_SIGNED(1).pdf",
            "Untitled document (2).docx",
            "Zoom Meeting Recording - July 4th.mp4",
            "zoom recordings - biology",
            "zoom recordings - biology.",
            "software ver-1.40.0-1.x86_64.rpm",
        ];
        names
    }
    fn convention_names() -> Vec<&'static str> {
        let names = vec![
            ".new-folder",
            ".new-folder",
            ".new-folder",
            "biology-notes-unit3-do-not-delete.pdf",
            "budget-2026-fixed.xlsx",
            "class-notes-fall-2025",
            "cool-music-project-wip",
            "cool-song-edit-final-v3-really-final.mp3",
            "desktop-clean-up-old",
            "do-not-delete",
            "draft-1-final-1.docx",
            "essay-new-v2-final.pdf",
            "family-photo-summer-vacation-1.jpeg",
            "img-4092.jpg",
            "important-read-this",
            "lab-report-chemistry-backup.docx",
            "math-homework-ch1-4",
            "math-homework-chapter-4.pages",
            "misc-stuff",
            "my-resume-2026.pdf",
            "new-folder-2",
            "new-folder-copy",
            "new-folder",
            "photos-1",
            "presentation-for-class-final2.pptx",
            "project-notes-idea-list.txt",
            "project-stuff-drafts",
            "receipts-invoices",
            "recipe-grandma-s-cookies.txt",
            // "screen-shot-2026-03-12-at-4-15-22-pm.png",
            "stuff-for-website.zip",
            "tax-documents-2025",
            "taxes-2025-signed-1.pdf",
            "untitled-document-2.docx",
            "zoom-meeting-recording-july-4th.mp4",
            "zoom-recordings-biology",
            "zoom-recordings-biology",
            "software-ver-1.40.0-1.x86-64.rpm",
        ];
        names
    }

    #[test]
    fn none_to_kebab() {
        let nones = no_convention_names();
        let mut kebabs_from_none = vec![];

        for snake in &nones {
            let kebab = to_kebab(snake, &Case::None).unwrap();
            kebabs_from_none.push(kebab);
        }

        for snake in &kebabs_from_none {
            println!("{snake}");
            assert!(check_for_kebab_case(snake));
        }

        let target = convention_names();

        // This is for debugging
        // let mut index = 0;
        // for keb in &nones {
        //     println!("\t{}\n{}\n{}", keb == &target[index], keb, &target[index]);
        //     index = index + 1;
        //     println!();
        // }
        // End of debugging

        assert_eq!(kebabs_from_none, target);
    }
    #[test]
    fn snake_to_kebab() {
        let nest = snake_names();
        let mut kebab_nest = vec![];

        for snake in &nest {
            let snake = to_kebab(snake, &Case::Snake).unwrap();
            kebab_nest.push(snake);
        }

        for new_snake in &kebab_nest {
            assert!(check_for_kebab_case(new_snake));
        }
        let target = kebab_ed_snake();

        // This is for debugging
        // let mut index = 0;
        // for new_keb in &kebab_nest {
        //     println!(
        //         "\t{}\n{}\n{}",
        //         new_keb == &target[index],
        //         new_keb,
        //         &target[index]
        //     );
        //     index = index + 1;
        //     println!();
        // }
        // end of debugging

        assert_eq!(kebab_nest, target);
    }
    #[test]
    fn all_kebab() {
        let examples = kebab_names();
        let kebab_tested: Vec<&str> = examples
            .iter()
            .map(|f| f.to_owned())
            .filter(|f| check_for_kebab_case(&f))
            .collect();

        assert_eq!(kebab_tested, examples)
    }
    #[test]
    fn not_kebab() {
        let kebab_tested: Vec<&str> = no_convention_names()
            .iter()
            .map(|f| f.to_owned())
            .filter(|f| check_for_kebab_case(&f))
            .collect();
        for i in &kebab_tested {
            println!("{:?}", i);
        }
        assert!(kebab_tested.is_empty())
    }
    #[test]
    fn all_snake() {
        let examples = snake_names();

        let snake_tested: Vec<&str> = examples
            .iter()
            .map(|f| f.to_owned())
            .filter(|f| check_for_snake_case(&f))
            .collect();
        assert_eq!(snake_tested, examples)
    }
    #[test]
    fn not_snake() {
        let snakes: Vec<&str> = no_convention_names()
            .iter()
            .map(|f| f.to_owned())
            .filter(|f| check_for_snake_case(&f))
            .collect();
        for i in &snakes {
            println!("{:?}", i);
        }
        assert!(snakes.is_empty())
    }
    #[test]
    fn all_camel() {
        let examples = camel_names();
        let camel_tested: Vec<&str> = examples
            .iter()
            .map(|f| f.to_owned())
            .filter(|f| check_for_camel_case(&f))
            .collect();

        assert_eq!(camel_tested, examples)
    }
    #[test]
    fn not_camel() {
        let camels: Vec<&str> = no_convention_names()
            .iter()
            .map(|f| f.to_owned())
            .filter(|f| check_for_camel_case(&f))
            .collect();
        for i in &camels {
            println!("{:?}", i);
        }
        assert!(camels.is_empty())
    }
    #[test]
    fn all_pascals() {
        let examples = pascal_names();

        let pascal_tested: Vec<&str> = examples
            .iter()
            .map(|f| f.to_owned())
            .filter(|f| check_for_pascal_case(&f))
            .collect();
        assert_eq!(pascal_tested, examples)
    }
    #[test]
    fn not_pascals() {
        let pascals: Vec<&str> = no_convention_names()
            .iter()
            .map(|f| f.to_owned())
            .filter(|f| check_for_pascal_case(&f))
            .collect();
        for i in &pascals {
            println!("{:?}", i);
        }
        assert!(pascals.is_empty())
    }
    #[test]
    fn check_casing() {
        let kebabs = kebab_names();
        for name in kebabs {
            assert_eq!(Case::Kebab, check_case(&name));
        }

        let snakes = snake_names();
        for snake in snakes {
            assert_eq!(Case::Snake, check_case(&snake))
        }

        let pascals = pascal_names();
        for pascal in pascals {
            assert_eq!(Case::Pascal, check_case(pascal))
        }

        let camels = camel_names();
        for camel in camels {
            assert_eq!(Case::Camel, check_case(camel))
        }
    }
}
