# fcom (file_combiner)

fcom (short for file_combiner) is a versatile Rust CLI tool designed to process folders and files, combining their contents into a single output file. It offers a range of features for file manipulation and directory analysis.

## Features

- Combine multiple files into a single output file
- Generate folder tree structures
- Create file lists
- Support for custom output formats (XML, Markdown, or custom templates)
- Option to add line numbers to file contents
- Ignore specific folders and filter by file extensions

## Installation

### Prerequisites

- Rust and Cargo (Install from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install))

### Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/fcom.git
   ```

2. Navigate to the project directory:
   ```bash
   cd fcom
   ```

3. Build and install the binary:
   ```bash
   cargo install --path .
   ```

This will install the `fcom` binary in your Cargo bin directory.

## Usage

fcom provides three main commands:

1. `combine`: Combines multiple files into a single output file
2. `tree`: Generates a folder tree structure
3. `list`: Creates a list of files in a folder

### combine

```bash
fcom combine <FOLDER_PATH> [OPTIONS]
```

Options:
- `--output <FILE>`: Specify the output file (default: output.txt)
- `--extensions <EXTENSIONS>...`: Specify file extensions to include
- `--ignore <FOLDERS>...`: Specify folders to ignore (default: .git,node_modules,__pycache__)
- `--add-line-numbers`: Add line numbers to file contents
- `--mode <MODE>`: Specify the output mode (xml, markdown, or custom) (default: xml)
- `--custom-output-template <FILE>`: Specify a custom output template file (required for custom mode)
- `--custom-file-template <FILE>`: Specify a custom file template file (required for custom mode)

### tree

```bash
fcom tree <FOLDER_PATH> [OPTIONS]
```

Options:
- `--output <FILE>`: Specify the output file (default: folder_tree.txt)
- `--extensions <EXTENSIONS>...`: Specify file extensions to include
- `--ignore <FOLDERS>...`: Specify folders to ignore (default: .git,node_modules,__pycache__)

### list

```bash
fcom list <FOLDER_PATH> [OPTIONS]
```

Options:
- `--output <FILE>`: Specify the output file (default: file_list.txt)
- `--extensions <EXTENSIONS>...`: Specify file extensions to include
- `--ignore <FOLDERS>...`: Specify folders to ignore (default: .git,node_modules,__pycache__)

## Examples

### Combine files in XML format

```bash
fcom combine /path/to/your/folder --output combined_output.txt --mode xml
```

### Generate a folder tree

```bash
fcom tree /path/to/your/folder --output folder_tree.txt
```

### Create a file list

```bash
fcom list /path/to/your/folder --output file_list.txt
```

### Combine files in Markdown format with line numbers

```bash
fcom combine /path/to/your/folder --output combined_output.md --mode markdown --add-line-numbers
```

### Use custom templates

```bash
fcom combine /path/to/your/folder --output custom_output.txt --mode custom --custom-output-template /path/to/output_template.txt --custom-file-template /path/to/file_template.txt
```

## Credits and Inspiration

This project was inspired by [Maximilian-Winter's file_combiner](https://github.com/Maximilian-Winter/file_combiner). We appreciate their work and the ideas it provided for this implementation.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
