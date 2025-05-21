# Rust CLI Parser Helper

`cli-parser-helper` is a lightweight and flexible library for parsing command-line arguments in Rust. It allows you to define CLI options with short and long forms, parse arguments, and retrieve their values or states. The library also generates a help message for your CLI application.

## Features

- **Register CLI Options**: Define options with short forms (e.g., `-c`), long forms (e.g., `--count`), and help text.
- **Parse Arguments**: Parse command-line arguments and retrieve normal arguments or option values.
- **Check Option States**: Determine if an option is enabled or retrieve its associated values.
- **Generate Help Text**: Automatically generate a help message based on registered options, with a customizable header and footer.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
cli-parser-helper = "0.1.3"
```

# Usage

Here’s an example of how to use the library:

```rust
use cli_parser_helper::CliOptionParser;

fn main() {
    let mut parser = CliOptionParser::new(
        "My CLI Application".to_string(),
        "For more information, visit https://example.com".to_string(),
    );

    // Register options
    parser.register_option(
        Some("-c".to_string()),
        Some("--count".to_string()),
        "Print only the count of selected lines",
        "count",
    );

    parser.register_option(
        Some("-C".to_string()),
        Some("--context".to_string()),
        "Print NUM lines of output context when given with --context=NUM",
        "context",
    );

    // Parse arguments
    let args = parser.parse();

    // Check if options are enabled
    if parser.is_enabled("count") {
        println!("Count option is enabled with values: {:?}", parser["count"]);
    }

    if parser.is_enabled("context") {
        println!("Context option is enabled with values: {:?}", parser["context"]);
    }

    // Print normal arguments
    println!("Normal arguments: {:?}", args);

    // Print help text
    println!("{}", parser.help_text());
}
```

# API Overview

`CliOptionParser`
`new(header: String, footer: String) -> CliOptionParser`
Creates a new parser with a customizable header and footer for the help text.

`register_option(short_form: Option<String>, long_form: Option<String>, help_text: &str, name: &str)`
Registers a new CLI option with its short form, long form, help text, and name.

`parse() -> Vec<String>`
Parses the command-line arguments and returns a list of normal arguments.

`is_enabled(name: &str) -> bool`
Checks if the option with the given name is enabled.

`get_option_values(name: &str) -> &Vec<String>`
Retrieves the values associated with the given option name.

`help_text() -> String`
Generates and returns the help text for the CLI options.

## Example Output

Given the following command-line input:

```sh
my_program -c123 --count=456 -C456 --context=712 normal_arg
```

The output would be:

```sh
Count option is enabled with values: ["123", "456"]
Context option is enabled with values: ["456", "712"]
Normal arguments: ["my_program", "normal_arg"]

My CLI Application

-c    --count         Print only the count of selected lines
-C    --context       Print NUM lines of output context when given with --context=NUM

For more information, visit https://example.com
```

# Testing

Run the tests using
`cargo test`

# License

This project is licensed under the MIT License.
