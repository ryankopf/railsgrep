# Railsgrep

Railsgrep is a command-line tool written in Rust, designed for searching specific patterns within files or directories (including subdirectories). It specializes in parsing and analyzing log files, especially useful for extracting and correlating data based on specific patterns such as request IDs in logs.

## Features

- Searches for a specified pattern in a file or recursively in directories.
- Extracts "tags" (e.g., request IDs) from the found lines.
- Retrieves and prints all lines containing any of the extracted tags, keeping the related entries together.

## Installation

### Prerequisites

- Rust Programming Language: Ensure you have Rust installed on your system. If not, you can install it from [here](https://www.rust-lang.org/tools/install).

### Building from Source

```sh
git clone https://github.com/ryankopf/railsgrep.git
cd railsgrep
cargo build --release
sudo mv target/release/railsgrep /usr/local/bin/
```

### Usage

To use Railsgrep, provide the path (file or directory) and the pattern you want to search for:

```sh
railsgrep <path> <pattern>
```

Example:

```sh
railsgrep ./logs 'ERROR'
```

This command will search through all files in the ./logs directory (and subdirectories) for the pattern 'ERROR', extract tags from matching lines, and then print all lines containing these tags.