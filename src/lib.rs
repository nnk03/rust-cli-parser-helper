#![allow(dead_code)]

use std::{collections::HashMap, ops::Index};

#[derive(Debug)]
struct Header {
    text: String,
}

#[derive(Debug)]
struct Footer {
    text: String,
}

impl Header {
    fn new(text: String) -> Header {
        Header { text }
    }
}

impl Footer {
    fn new(text: String) -> Footer {
        Footer { text }
    }
}

type Name = String;

#[derive(Debug)]
struct CliOption {
    long_form: Option<String>,
    short_form: Option<String>,
    help_text: String,
    name: Name,
}

#[derive(Debug)]
struct OptionValue {
    is_enabled: bool,
    values: Vec<String>,
}

impl OptionValue {
    fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    fn values(&self) -> &Vec<String> {
        &self.values
    }
}

/// Struct to parse the option arguments
/// Ignores invalid arguments passed
pub struct CliOptionParser {
    name_to_cli_option_map: HashMap<Name, CliOption>,
    name_to_option_value_map: HashMap<Name, OptionValue>,
    short_form_to_name_map: HashMap<String, Name>,
    long_form_to_name_map: HashMap<String, Name>,
    empty_option_list: Vec<String>,
    header: Header,
    footer: Footer,
}

impl Index<&str> for CliOptionParser {
    type Output = Vec<String>;

    fn index(&self, index: &str) -> &Self::Output {
        self.get_option_values(index)
    }
}

impl CliOptionParser {
    /// Gives a new instance of CliOptionParser
    pub fn new(header: String, footer: String) -> CliOptionParser {
        CliOptionParser {
            name_to_cli_option_map: HashMap::new(),
            name_to_option_value_map: HashMap::new(),
            short_form_to_name_map: HashMap::new(),
            long_form_to_name_map: HashMap::new(),
            empty_option_list: vec![],
            header: Header { text: header },
            footer: Footer { text: footer },
        }
    }

    /// sets the enabled flag to true for the flag `option`
    fn enable_option(&mut self, option_name: String) {
        let option_value_entry = self
            .name_to_option_value_map
            .entry(option_name.to_string())
            .or_insert(OptionValue {
                is_enabled: false,
                values: vec![],
            });

        option_value_entry.is_enabled = true;
    }

    /// appends the value given to the corresponding flag `option`
    fn add_value_to_option(&mut self, option_name: String, value: String) {
        let option_value_entry = self
            .name_to_option_value_map
            .entry(option_name.to_string())
            .or_insert(OptionValue {
                is_enabled: false,
                values: vec![],
            });

        option_value_entry.values.push(value);
        option_value_entry.is_enabled = true;
    }

    fn parse_from(&mut self, args: Vec<String>) -> Vec<String> {
        let mut arguments: Vec<String> = vec![];

        for arg in args {
            if arg.starts_with("--") {
                if arg.contains("=") {
                    // example --hello=world
                    let mut arg_split = arg.split("=");
                    let option = arg_split.next().unwrap(); // --hello
                    let value = arg_split.next().unwrap(); // world

                    if !self.long_form_to_name_map.contains_key(option) {
                        continue;
                    }

                    let option_name = &self.long_form_to_name_map[option];

                    // add the value to option
                    self.add_value_to_option(option_name.to_string(), value.to_string());
                } else {
                    if !self.long_form_to_name_map.contains_key(&arg) {
                        continue;
                    }

                    let option_name = &self.long_form_to_name_map[&arg];
                    self.enable_option(option_name.to_string());
                }
            } else if arg.starts_with("-") {
                // if length is greater than 1, like -lHelloWorld
                if arg.len() > 1 {
                    let (option, value) = arg.split_at(2); // left = -l, right = HelloWorld
                    if !self.short_form_to_name_map.contains_key(option) {
                        continue;
                    }
                    let option_name = &self.short_form_to_name_map[option];
                    self.add_value_to_option(option_name.to_string(), value.to_string());
                } else {
                    if !self.short_form_to_name_map.contains_key(&arg) {
                        continue;
                    }

                    let option_name = &self.short_form_to_name_map[&arg];
                    self.enable_option(option_name.to_string());
                }
            } else {
                arguments.push(arg);
            }
        }

        arguments
    }

    /// function to parse and then returns the normal arguments
    pub fn parse(&mut self) -> Vec<String> {
        // function to parse
        self.parse_from(std::env::args().collect())
    }

    /// Checks if an option with the given `name` is present/enabled
    pub fn is_enabled(&self, name: &str) -> bool {
        // return false if invalid option
        if !self.name_to_option_value_map.contains_key(name) {
            return false;
        }

        self.name_to_option_value_map[name].is_enabled()
    }

    /// if an option with the given `name` is enabled, returns the reference to value list
    /// else returns an empty list
    pub fn get_option_values(&self, name: &str) -> &Vec<String> {
        if !self.is_enabled(name) {
            return &self.empty_option_list;
        }

        &self.name_to_option_value_map[name].values()
    }

    /// Registers an Option with the given short form, long form, help text with the name `name`
    /// the option can be accessed by using the `name`
    pub fn register_option(
        &mut self,
        short_form: Option<String>,
        long_form: Option<String>,
        help_text: &str,
        name: &str,
    ) {
        let help_text = help_text.to_string();
        let name = name.to_string();

        if self.name_to_cli_option_map.contains_key(&name) {
            panic!("name : {name} already registered as option");
        }

        if short_form.is_none() && long_form.is_none() {
            panic!("Both long form and short form cannot be none for name : {name}");
        }

        if let Some(short_form_name) = short_form.as_ref() {
            if self
                .short_form_to_name_map
                .contains_key(&short_form_name.clone())
            {
                panic!("Short form of {short_form_name} already defined");
            }

            self.short_form_to_name_map
                .insert(short_form_name.clone(), name.clone());
        }

        if let Some(long_form_name) = long_form.as_ref() {
            if self
                .long_form_to_name_map
                .contains_key(&long_form_name.clone())
            {
                panic!("Short form of {long_form_name} already defined");
            }

            self.long_form_to_name_map
                .insert(long_form_name.clone(), name.clone());
        }

        self.name_to_cli_option_map.insert(
            name.clone(),
            CliOption {
                long_form,
                short_form,
                help_text,
                name: name.clone(),
            },
        );

        self.name_to_option_value_map.insert(
            name.clone(),
            OptionValue {
                is_enabled: false,
                values: vec![],
            },
        );
    }

    /// function to return help text when asked
    pub fn help_text(&self) -> String {
        // display help text
        let mut help_text = format!("{}\n\n", self.header.text);

        for (_, cli_option) in &self.name_to_cli_option_map {
            match cli_option.short_form.as_ref() {
                Some(short) => help_text += &format!("{}    ", short),
                None => help_text += &format!("     "),
            }

            match cli_option.long_form.as_ref() {
                Some(long) => help_text += &format!("{}    ", long),
                None => help_text += &format!("     "),
            }

            help_text += &format!("     {}\n", cli_option.help_text.replace('\n', "\n\t\t\t"));
        }

        help_text += "\n";

        help_text += &format!("{}\n\n", self.footer.text);

        help_text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_parsing() {
        let mut cli_option_parser =
            CliOptionParser::new("header".to_string(), "footer".to_string());
        cli_option_parser.register_option(
            Some("-c".to_string()),
            Some("--count".to_string()),
            "print only count of selected lines",
            "count",
        );

        cli_option_parser.register_option(
            Some("-C".to_string()),
            Some("--context".to_string()),
            "print NUM lines of output contex when\n given with --context=NUM",
            "context",
        );

        let mock_args = vec![
            "program_name".to_string(),
            "-c123".to_string(),
            "--count=456".to_string(),
            "-C456".to_string(),
            "--context=712".to_string(),
        ];

        let arguments = cli_option_parser.parse_from(mock_args);

        assert_eq!(arguments, vec!["program_name"]);

        assert!(cli_option_parser.is_enabled("count"));
        assert!(cli_option_parser.is_enabled("context"));

        assert!(cli_option_parser["count"].len() == 2);
        assert_eq!(cli_option_parser["count"], vec!["123", "456"]);

        assert!(cli_option_parser["context"].len() == 2);
        assert_eq!(cli_option_parser["context"], vec!["456", "712"]);

        let help_text = cli_option_parser.help_text();

        assert!(help_text.contains("header"));
        assert!(help_text.contains("footer"));
        assert!(help_text.contains("-c    --count         print only count of selected lines"));
        assert!(help_text.contains("-C    --context         print NUM lines of output contex when\n\t\t\t given with --context=NUM"));
    }
}
