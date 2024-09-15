use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

mod combiner;
use combiner::FolderProcessOptions;

#[derive(Parser)]
#[clap(author, version, about = "A tool for combining and analyzing files in a directory", long_about = None)]
#[clap(after_help = "Example usage:
  fcom combine /path/to/folder -o output.txt -e rs,toml -i target -l -m markdown
  fcom tree /path/to/folder -o tree.txt
  fcom list /path/to/folder -o list.txt")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "Combine files in a folder")]
    #[clap(
        long_about = "Combine files in a folder, with options to filter by extension, ignore certain files/folders, add line numbers, and choose output format"
    )]
    Combine {
        #[clap(help = "Path to the folder to process")]
        folder_path: PathBuf,
        #[clap(
            short = 'o',
            long,
            default_value = "output.txt",
            help = "Name of the output file"
        )]
        output: String,
        #[clap(
            short = 'e',
            long,
            value_delimiter = ',',
            help = "File extensions to include (comma-separated)"
        )]
        extensions: Option<Vec<String>>,
        #[clap(
            short = 'i',
            long,
            value_delimiter = ',',
            help = "Folders or files to ignore (comma-separated)"
        )]
        ignore: Option<Vec<String>>,
        #[clap(short = 'l', long, help = "Add line numbers to the output")]
        add_line_numbers: bool,
        #[clap(
            short = 'm',
            long,
            default_value = "xml",
            help = "Output mode: xml, markdown, or custom"
        )]
        mode: String,
        #[clap(long, help = "Path to custom output template file")]
        custom_output_template: Option<PathBuf>,
        #[clap(long, help = "Path to custom file template file")]
        custom_file_template: Option<PathBuf>,
    },
    #[clap(about = "Generate a folder tree")]
    Tree {
        #[clap(help = "Path to the folder to process")]
        folder_path: PathBuf,
        #[clap(
            short = 'o',
            long,
            default_value = "folder_tree.txt",
            help = "Name of the output file"
        )]
        output: String,
        #[clap(
            short = 'e',
            long,
            help = "File extensions to include (comma-separated)"
        )]
        extensions: Option<Vec<String>>,
        #[clap(
            short = 'i',
            long,
            default_value = ".git,node_modules,__pycache__",
            help = "Folders to ignore (comma-separated)"
        )]
        ignore: Vec<String>,
    },
    #[clap(about = "Generate a list of files")]
    List {
        #[clap(help = "Path to the folder to process")]
        folder_path: PathBuf,
        #[clap(
            short = 'o',
            long,
            default_value = "file_list.txt",
            help = "Name of the output file"
        )]
        output: String,
        #[clap(
            short = 'e',
            long,
            help = "File extensions to include (comma-separated)"
        )]
        extensions: Option<Vec<String>>,
        #[clap(
            short = 'i',
            long,
            default_value = ".git,node_modules,__pycache__",
            help = "Folders to ignore (comma-separated)"
        )]
        ignore: Vec<String>,
    },
}

fn read_gitignore(folder_path: &Path) -> Vec<String> {
    let gitignore_path = folder_path.join(".gitignore");
    let mut ignore_patterns = vec![
        ".git".to_string(),
        "node_modules".to_string(),
        "__pycache__".to_string(),
        "target".to_string(),
    ];

    if gitignore_path.exists() {
        if let Ok(file) = File::open(gitignore_path) {
            let reader = BufReader::new(file);
            for pattern in reader.lines().map_while(Result::ok) {
                let trimmed = pattern.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    ignore_patterns.push(trimmed.to_string());
                }
            }
        } else {
            eprintln!("Failed to open .gitignore file");
        }
    }

    ignore_patterns
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Combine {
            folder_path,
            output,
            extensions,
            ignore,
            add_line_numbers,
            mode,
            custom_output_template,
            custom_file_template,
        } => {
            let mut ignore_patterns = read_gitignore(folder_path);
            if let Some(additional_ignores) = ignore {
                ignore_patterns.extend(additional_ignores.iter().cloned());
            }

            let options = FolderProcessOptions {
                folder_path,
                output_file: output,
                file_extensions: extensions.as_ref().map(|e| e.as_slice()),
                ignore_patterns: &ignore_patterns,
                add_line_numbers: *add_line_numbers,
                mode,
                custom_output_template: custom_output_template.as_ref(),
                custom_file_template: custom_file_template.as_ref(),
            };

            combiner::process_folder(options);
        }
        Commands::Tree {
            folder_path,
            output,
            extensions,
            ignore,
        } => {
            let tree = combiner::create_folder_tree(
                folder_path,
                extensions.as_ref().map(|e| e.as_slice()),
                ignore,
            );
            std::fs::write(output, tree).expect("Unable to write folder tree to file");
            println!("Folder tree has been generated and saved to '{}'.", output);
        }
        Commands::List {
            folder_path,
            output,
            extensions,
            ignore,
        } => {
            let list = combiner::create_file_list(
                folder_path,
                extensions.as_ref().map(|e| e.as_slice()),
                ignore,
            );
            std::fs::write(output, list).expect("Unable to write file list to file");
            println!("File list has been generated and saved to '{}'.", output);
        }
    }
}
