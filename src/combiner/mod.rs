use chrono::Local;
use std::fs;
use std::path::{Path, PathBuf};

mod template;
use template::CombinerTemplate;

const XML_OUTPUT_TEMPLATE: &str = r#"<file_overview>
Total files: {TOTAL_FILES}
Date generated: {DATE_GENERATED}
Folder Structure:
{FOLDER_TREE}

Files included:
{FILES_INCLUDED}
</file_overview>

{FILE_CONTENTS}"#;

const XML_FILE_TEMPLATE: &str = r#"<file path="{FILE_PATH}" lines="{LINES_COUNT}" modified="{MODIFIED_TIME}">
{FILE_CONTENT}
</file>

"#;

const MARKDOWN_OUTPUT_TEMPLATE: &str = r#"# File Overview

- **Total files:** {TOTAL_FILES}
- **Date generated:** {DATE_GENERATED}

## Folder Structure

```
{FOLDER_TREE}
```

## Files Included

{FILES_INCLUDED}


## Files Contents

---
{FILE_CONTENTS}"#;

const MARKDOWN_FILE_TEMPLATE: &str = r#"### {FILE_NAME}

- **Path:** `{FILE_PATH}`
- **Lines:** {LINES_COUNT}
- **Modified:** {MODIFIED_TIME}

```
{FILE_CONTENT}
```

---

"#;

pub struct FolderProcessOptions<'a> {
    pub folder_path: &'a Path,
    pub output_file: &'a str,
    pub file_extensions: Option<&'a [String]>,
    pub ignore_patterns: &'a [String],
    pub add_line_numbers: bool,
    pub mode: &'a str,
    pub custom_output_template: Option<&'a PathBuf>,
    pub custom_file_template: Option<&'a PathBuf>,
}

pub fn process_folder(options: FolderProcessOptions) {
    if !options.folder_path.is_dir() {
        println!(
            "Error: The folder '{}' does not exist.",
            options.folder_path.display()
        );
        return;
    }

    let all_files = get_all_files(
        options.folder_path,
        options.file_extensions,
        options.ignore_patterns,
    );

    let (output_template, file_template) = match options.mode.to_lowercase().as_str() {
        "xml" => (
            CombinerTemplate::from_string(XML_OUTPUT_TEMPLATE),
            CombinerTemplate::from_string(XML_FILE_TEMPLATE),
        ),
        "markdown" => (
            CombinerTemplate::from_string(MARKDOWN_OUTPUT_TEMPLATE),
            CombinerTemplate::from_string(MARKDOWN_FILE_TEMPLATE),
        ),
        "custom" => {
            if options.custom_output_template.is_none() || options.custom_file_template.is_none() {
                panic!("Custom mode requires both custom output and file templates.");
            }
            (
                CombinerTemplate::from_file(options.custom_output_template.unwrap()),
                CombinerTemplate::from_file(options.custom_file_template.unwrap()),
            )
        }
        _ => panic!(
            "Invalid mode: {}. Choose 'xml', 'markdown', or 'custom'.",
            options.mode
        ),
    };

    let folder_tree = create_folder_tree(
        options.folder_path,
        options.file_extensions,
        options.ignore_patterns,
    );
    let files_included = all_files
        .iter()
        .map(|f| {
            format!(
                "- {}",
                f.strip_prefix(options.folder_path).unwrap().display(),
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let mut all_file_contents = Vec::new();
    for file_path in &all_files {
        let relative_path = file_path.strip_prefix(options.folder_path).unwrap();
        let metadata = fs::metadata(file_path).unwrap();
        let mod_time = metadata.modified().unwrap();
        let mod_time = chrono::DateTime::<Local>::from(mod_time)
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        match fs::read_to_string(file_path) {
            Ok(content) => {
                let line_count = content.lines().count();
                let formatted_content = if options.add_line_numbers {
                    content
                        .lines()
                        .enumerate()
                        .map(|(i, line)| format!("{:<6}| {}", i + 1, line))
                        .collect::<Vec<_>>()
                        .join("\n")
                } else {
                    content
                };

                let file_content = file_template.generate_output_file_content(
                    &[
                        ("FILE_PATH", relative_path.display().to_string()),
                        (
                            "FILE_NAME",
                            file_path.file_name().unwrap().to_str().unwrap().to_string(),
                        ),
                        ("LINES_COUNT", line_count.to_string()),
                        ("MODIFIED_TIME", mod_time),
                        ("FILE_CONTENT", formatted_content.trim().to_string()),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                );
                all_file_contents.push(file_content);
            }
            Err(e) => {
                println!(
                    "Warning: Couldn't read file '{}': {}",
                    file_path.display(),
                    e
                );
                continue;
            }
        }
    }

    let output_content = output_template.generate_output_file_content(
        &[
            ("TOTAL_FILES", all_files.len().to_string()),
            (
                "DATE_GENERATED",
                Local::now().format("%Y-%m-d %H:%M:%S").to_string(),
            ),
            ("FOLDER_TREE", folder_tree.trim().to_string()),
            ("FILES_INCLUDED", files_included),
            ("FILE_CONTENTS", all_file_contents.join("")),
        ]
        .iter()
        .cloned()
        .collect(),
    );

    fs::write(options.output_file, output_content).expect("Unable to write output file");
    println!(
        "All files have been processed and combined into '{}' using {} mode.",
        options.output_file, options.mode
    );
}

pub fn create_folder_tree(
    path: &Path,
    file_extensions: Option<&[String]>,
    ignore_folders: &[String],
) -> String {
    create_folder_tree_inner(path, file_extensions, ignore_folders, String::new())
}

fn create_folder_tree_inner(
    path: &Path,
    file_extensions: Option<&[String]>,
    ignore_folders: &[String],
    prefix: String,
) -> String {
    let mut tree = String::new();
    let mut contents: Vec<_> = fs::read_dir(path)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| !ignore_folders.contains(&entry.file_name().to_string_lossy().into_owned()))
        .collect();
    contents.sort_by_key(|a| (!a.path().is_dir(), a.file_name()));

    for (i, entry) in contents.iter().enumerate() {
        let is_last = i == contents.len() - 1;
        let file_name = entry.file_name();
        let file_path = entry.path();

        if file_path.is_dir() {
            tree.push_str(&format!(
                "{}{} {}/\n",
                prefix,
                if is_last { "└──" } else { "├──" },
                file_name.to_string_lossy()
            ));
            let new_prefix = format!("{}{}   ", prefix, if is_last { " " } else { "│" });
            tree.push_str(&create_folder_tree_inner(
                &file_path,
                file_extensions,
                ignore_folders,
                new_prefix,
            ));
        } else if file_extensions.is_none()
            || file_extensions
                .unwrap()
                .iter()
                .any(|ext| file_name.to_string_lossy().ends_with(ext))
        {
            tree.push_str(&format!(
                "{}{} {}\n",
                prefix,
                if is_last { "└──" } else { "├──" },
                file_name.to_string_lossy()
            ));
        }
    }

    tree
}

pub fn create_file_list(
    folder_path: &Path,
    file_extensions: Option<&[String]>,
    ignore_folders: &[String],
) -> String {
    get_all_files(folder_path, file_extensions, ignore_folders)
        .iter()
        .map(|f| format!("- {}", f.strip_prefix(folder_path).unwrap().display()))
        .collect::<Vec<_>>()
        .join("\n")
}

fn get_all_files(
    folder_path: &Path,
    file_extensions: Option<&[String]>,
    ignore_folders: &[String],
) -> Vec<PathBuf> {
    let mut all_files = Vec::new();
    for entry in fs::read_dir(folder_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            if !ignore_folders.contains(&path.file_name().unwrap().to_string_lossy().into_owned()) {
                all_files.extend(get_all_files(&path, file_extensions, ignore_folders));
            }
        } else if file_extensions.is_none()
            || file_extensions.unwrap().iter().any(|ext| {
                path.extension()
                    .and_then(|e| e.to_str())
                    .map_or(false, |e| e == ext.trim_start_matches("."))
            })
        {
            all_files.push(path);
        }
    }
    all_files.sort();
    all_files
}
