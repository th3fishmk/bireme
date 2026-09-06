use crate::Case;
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

pub fn to_kebab(target: &str) -> Option<String> {
    none_to_kebab(target)
}

fn none_to_kebab(target: &str) -> Option<String> {
    let double_dots = Regex::new(r"\.{2,}").unwrap();
    let all_invalids = Regex::new(r"[^a-z0-9\-\_]").unwrap();
    let double_dash = Regex::new(r"\-{2,}").unwrap();

    let target = double_dots.replace_all(target, ".").to_string();

    let parts: Vec<_> = target.split(".").collect();
    let mut processed = vec![];

    for part in parts {
        let mut pice = part.to_lowercase();
        pice = pice.replace("_", "-");
        pice = pice.trim().to_string();
        pice = pice.replace(" ", "-");
        // pice = pice.trim().to_string();

        pice = all_invalids.replace_all(&pice, "").to_string();
        pice = double_dash.replace_all(&pice, "-").to_string();
        pice = pice.trim_start_matches("-").to_string();
        pice = pice.trim_end_matches("-").to_string();

        processed.push(pice);
    }

    let mut result = processed.join(".");
    result = result.trim_end_matches(".").to_string();
    if check_for_kebab_case(&result) {
        // println!("kebab || {}", result);
        Some(result)
    } else {
        // println!("!kebab || {}", result);
        None
    }
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
    fn conventioned_names() -> Vec<&'static str> {
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
            "recipe-grandmas-cookies.txt",
            // "screen-shot-2026-03-12-at-4-15-22-pm.png",
            "stuff-for-website.zip",
            "tax-documents-2025",
            "taxes-2025-signed1.pdf",
            "untitled-document-2.docx",
            "zoom-meeting-recording-july-4th.mp4",
            "zoom-recordings-biology",
            "zoom-recordings-biology",
            "software-ver-1.40.0-1.x86-64.rpm",
        ];
        names
    }

    #[test]
    fn convention_ate() {
        let no_convention = no_convention_names();
        let convention = conventioned_names();

        let mut res = vec![];

        for i in no_convention {
            res.push(none_to_kebab(&i.to_string()));
        }

        for item in &res {
            let item_name = item.clone().unwrap();
            let item_name = item_name.as_str();
            let is_kebab = check_for_kebab_case(item_name);
            // println!("{:?}", &item);
            // println!("{item_name}");
            assert_eq!(is_kebab, true);
        }

        let mut all_kebab = vec![];
        for i in &res {
            let i = i.clone().unwrap();
            all_kebab.push(i);
        }
        let all_kebab: Vec<_> = all_kebab.iter().map(|f| f.as_str()).collect();
        // assert_eq!(all_kebab, convention);
        let mut index = 0;
        for i in all_kebab {
            assert_eq!(i, convention[index]);
            index = index + 1;
        }
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
