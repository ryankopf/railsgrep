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



## Use Case

Rails logs contain lines like:
```
[019b0183-9f1c-11ef-b1f1-81ed7c584657] [50.222.253.13] Completed 200 OK in 27ms (Views: 15.4ms | ActiveRecord: 8.3ms | Allocations: 11936)
[21b001a3-72ca-414e-b13c-a175bdf3a766] [125.125.171.14] Started GET "/topics/340/topics/42581?page=1" for 185.191.171.14 at 2023-12-20 20:03:53 +0000
[21b001a3-72ca-414e-b13c-a175bdf3a766] [125.125.171.14] Processing by TopicsController#show as HTML
```

Sometimes these lines will be mixed up, like:
```
[21b001a3-72ca-414e-b13c-a175bdf3a766] [125.125.171.14] Started GET "/topics/340/topics/42581?page=1" for 185.191.171.14 at 2023-12-20 20:03:53 +0000
[019b0183-9f1c-11ef-b1f1-81ed7c584657] [50.222.253.13] Completed 200 OK in 27ms (Views: 15.4ms | ActiveRecord: 8.3ms | Allocations: 11936)
[21b001a3-72ca-414e-b13c-a175bdf3a766] [125.125.171.14] Processing by TopicsController#show as HTML
```

Or sometimes you may have accidentally logged newline characters:
```
[21b001a3-72ca-414e-b13c-a175bdf3a766] [125.125.171.14] Received from payment processor {
    name: 'John',
    card_id: '1234567'
}
```

But you need to search for "payment processor". You can use railsgreg to find these lines.

```
railsgrep ./log/ 'payment processor'
```