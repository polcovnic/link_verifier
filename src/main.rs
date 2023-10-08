use clap::Parser;
use crate::commands::command_interface::Command;
use crate::commands::extract_links_command::ExtractLinksCommand;
use colored::Colorize;

mod commands;

use crate::commands::verify_file_links_command::{VerifyFileLinksCommand, FileLinksResult};
use crate::commands::verify_url_links_command::{UrlLinksResult, VerifyUrlLinksCommand};

#[derive(Parser, Debug)]
#[command(author = "polcovnic", version = "1.0", about = "Verifies links in given markdown files")]
struct Args {
    #[arg(short, long, required = true)]
    file: String,
}


fn pretty_print(file_links_result: FileLinksResult, url_links_result: UrlLinksResult) {
    println!("{}", "Ok file links:".bold());
    for ok_file_link in file_links_result.ok_links {
        println!("{}", ok_file_link.to_str().unwrap().green());
    }
    println!("\n{}", "Broken file links:".bold());
    for broken_file_link in file_links_result.broken_links {
        print!("{}  ", broken_file_link.path.to_str().unwrap().red());
        if broken_file_link.suggestions.is_empty() {
            println!();
            continue;
        }
        let suggestions_str = broken_file_link.suggestions.iter()
            .map(|s| s.to_str()
                .unwrap())
            .collect::<Vec<&str>>()
            .join(", ");
        println!("Maybe you can try: {}", suggestions_str.yellow());
    }
    println!("\n{}", "Ok url links:".bold());
    for ok_url_link in url_links_result.valid_urls {
        println!("{}", ok_url_link.green());
    }
    println!("\n{}", "Broken url links:".bold());
    for broken_url_link in url_links_result.invalid_urls {
        println!("{}", broken_url_link.red());
    }
}

fn main() {
    let args = Args::parse();

    let extract = ExtractLinksCommand::new(args.file);
    let links = extract.execute();
    let verify_file_links = VerifyFileLinksCommand::new(links.local_files);
    let file_links_result = verify_file_links.execute();
    let verify_url_links = VerifyUrlLinksCommand::new(links.external_links);
    let url_links_result = verify_url_links.execute();
    pretty_print(file_links_result, url_links_result);
}
