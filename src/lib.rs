#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug)]
struct Header {
    text: String,
}

#[derive(Debug)]
struct Footer {
    text: String,
}

impl Header {
    pub fn new(text: String) -> Header {
        Header { text }
    }
}

impl Footer {
    pub fn new(text: String) -> Footer {
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
}

impl CliOptionParser {
    /// Gives a new instance of CliOptionParser
    pub fn new() -> CliOptionParser {
        CliOptionParser {
            name_to_cli_option_map: HashMap::new(),
            name_to_option_value_map: HashMap::new(),
            short_form_to_name_map: HashMap::new(),
            long_form_to_name_map: HashMap::new(),
            empty_option_list: vec![],
        }
    }

    pub fn parse(&self) {
        // function to parse
    }

    /// Checks if an option with the given `name` is present
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
}
