// commands/verify_file_links_command.rs

use std::fs;
use std::path::{Path, PathBuf};
use super::command_interface::Command;
use strsim::levenshtein;


const MAX_THRESHOLD: usize = 5; // Max amount of broken characters

pub struct VerifyFileLinksCommand {
    paths: Vec<PathBuf>,
}

impl VerifyFileLinksCommand {
    pub fn new(paths: Vec<PathBuf>) -> Self {
        VerifyFileLinksCommand { paths }
    }
}


#[derive(Debug)]
pub struct BrokenLink {
    pub path: PathBuf,
    pub suggestions: Vec<PathBuf>,
}
#[derive(Debug)]
pub struct FileLinksResult {
    pub broken_links: Vec<BrokenLink>,
    pub ok_links: Vec<PathBuf>,
}

impl Command for VerifyFileLinksCommand {
    type Output = FileLinksResult;
    fn execute(&self) -> FileLinksResult {
        let mut broken_links = Vec::new();
        let mut ok_links = Vec::new();
        for path in &self.paths {
            if fs::metadata(path).is_ok() {
                ok_links.push(path.to_owned());
            } else {
                let similar_paths = find_similar_paths(&path.to_string_lossy(), ".");
                broken_links.push(BrokenLink {
                    path: path.to_owned(),
                    suggestions: similar_paths
                });

            }
        }
        FileLinksResult {
            broken_links,
            ok_links,
        }
    }
}

fn find_similar_paths(target: &str, directory: &str) -> Vec<PathBuf> {
    let mut similar_paths = Vec::new();
    let max_threshold = 5; // Max amount of broken characters

    fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&Path)) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    visit_dirs(&path, cb);
                } else {
                    cb(&path);
                }
            }
        }
    }

    visit_dirs(Path::new(directory), &mut |path| {
        let path_str = path.to_string_lossy().to_string();
        let threshold = std::cmp::min(path_str.len() / MAX_THRESHOLD, MAX_THRESHOLD);
        if levenshtein(target, &path_str) <= threshold {
            similar_paths.push(PathBuf::from(path_str));
        }
    });

    similar_paths
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::path::PathBuf;

    #[test]
    fn test_verify_links() {
        let existing_file = "existing_file.md";
        let missing_file = "missing_file.md";

        File::create(existing_file).unwrap();

        let paths = vec![
            PathBuf::from(existing_file),
            PathBuf::from(missing_file),
        ];

        let command = VerifyFileLinksCommand::new(paths);
        let result = command.execute();

        assert_eq!(result.ok_links.len(), 1);
        assert_eq!(result.broken_links.len(), 1);
        assert!(result.ok_links.contains(&PathBuf::from(existing_file)));

        std::fs::remove_file(existing_file).unwrap();
    }
}
