use std::fs;
use std::path::PathBuf;

use regex::Regex;
use url::Url;

use super::command_interface::Command;
pub struct ExtractLinksCommand {
    path: PathBuf,
}

#[derive(Debug)]
pub struct ExtractedLinks {
    pub local_files: Vec<PathBuf>,
    pub external_links: Vec<String>,
}

impl ExtractLinksCommand {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        ExtractLinksCommand { path: path.into() }
    }
}

impl Command for ExtractLinksCommand {
    type Output = ExtractedLinks;
    fn execute(&self) -> ExtractedLinks {
        let content = fs::read_to_string(&self.path).expect("Failed to read the file");

        let md_link_regex = Regex::new(r"\[(?:[^\[\]]*)\]\((.*?)\)").expect("Failed to compile regex");

        let mut local_files = Vec::new();
        let mut external_links = Vec::new();

        for cap in md_link_regex.captures_iter(&content) {
            if let Some(link) = cap.get(1) {
                let link_str = link.as_str();
                if Url::parse(link_str).is_ok() {
                        external_links.push(link_str.to_string());
                } else {
                        local_files.push(PathBuf::from(link_str));
                }
            }
        }

        ExtractedLinks {
            local_files,
            external_links,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    #[test]
    fn test_extract_links() {
        let test_file_path = "test_extract_links.md";
        let test_content = r#"
        This is a markdown file.
        Here's a [local link](./another_file.md).
        Here's another [local link](./documents/document.md).
        This is an [external link](https://www.google.com).
        And here's another [external link](https://www.openai.com).
        "#;

        let mut test_file = File::create(test_file_path).unwrap();
        test_file.write_all(test_content.as_bytes()).unwrap();

        let command = ExtractLinksCommand::new(test_file_path.to_string());
        let result = command.execute();

        assert_eq!(result.local_files.len(), 2);
        assert_eq!(result.external_links.len(), 2);

        assert!(result.local_files.contains(&PathBuf::from("./another_file.md")));
        assert!(result.local_files.contains(&PathBuf::from("./documents/document.md")));

        assert!(result.external_links.contains(&"https://www.google.com".to_string()));
        assert!(result.external_links.contains(&"https://www.openai.com".to_string()));

        std::fs::remove_file(test_file_path).unwrap();
    }


}

