pub mod interagibles;
pub mod visitables;

use self::visitables::*;
use self::interagibles::*;
use std::collections::HashMap;
use std::io::{self, Write};
use std::process;
use regex::{Regex};

pub struct Game<T: Visitable> {
    sections: HashMap<String, T>,
    start_section_tag: String,
    current_section_tag : String,
}

/// A trait which allows the `Game` to peform IO on the console.
/// Any trait can be provided and implemented, so that IO is possible on browsers, ...
pub trait ConsoleIO {
    /// Just read the command and return it (with no trailing newlines or space around)
    fn read_command(&self) -> String {
        let mut player_input = String::new();
        print!("> ");
        io::stdout().flush().unwrap(); // Or it won't print, as stdout is line-buffered
        match io::stdin().read_line(&mut player_input) {
            Ok(_) => (),
            Err(error) => panic!("Input error: {}", error),
        };
        player_input.to_lowercase().trim().to_string()
    }
    /// Write the passed string, adding a _newline_
    fn write_line(&self, line: &str) {
        println!("{}", line);
    }
}

impl<T: Visitable> Game<T> {
    pub fn new(i_sections: HashMap<String, T>, i_start_section_tag: String) -> Self {
        Game {
            sections: i_sections,
            start_section_tag: i_start_section_tag,
            current_section_tag: String::new(),
        }
    }

    pub fn run(&mut self) {
        self.current_section_tag = self.start_section_tag.clone();
        let mut previter_section_tag = String::new();

        let interaction_regex = Regex::new(r"(\w+)\s+(\w+)").unwrap();
        loop {
            let current_section = &self.sections[&self.current_section_tag];
            let cs_interagibles = current_section.get_interagibles();

            if self.current_section_tag != previter_section_tag {
                self.write_position();
            }
            previter_section_tag = self.current_section_tag.clone();

            let command = self.read_command();

            match command.as_str() {
                dir if (dir == "n" || dir == "s" || dir == "w" || dir == "e") => {
                    match current_section.exit(&EXITDIRS[dir]) {
                        Exit::Visitable(s) => {
                            self.current_section_tag = s;
                        }
                        Exit::Closed(s) => {
                            self.write_line(&s);
                        }
                        Exit::None => {
                            self.write_line("No exit this way.");
                        }
                    };
                }
                irx if interaction_regex.is_match(irx) => {
                    let caps = interaction_regex.captures(irx).unwrap();
                    let interaction = match INTERACTIONS.get(caps.get(1).unwrap().as_str()) {
                        Some(intn) => intn,
                        None => {
                            self.write_line("Unknown command.");
                            continue;
                        }
                    };
                    let target = cs_interagibles.get(caps.get(2).unwrap().as_str());
                    match target {
                        Some(_) => {},
                        None => {
                            self.write_line("Invalid target for action.");
                            continue;
                        }
                    };
                    self.write_line(&format!("Interacting {:?} - {:?}", interaction, target));
                }
                "pos" => {
                    self.write_position();
                }
                cmd if (cmd == "q" || cmd == "quit") => {
                    self.write_line("See you!");
                    process::exit(0);
                }
                _ => {
                    self.write_line("Unknown command.");
                }
            };
        }
    }

    fn write_position(&self) {
        let current_section = &self.sections[&self.current_section_tag];
        self.write_line(&format!("You are in the {}", current_section.get_name()));
        self.write_line(&current_section.get_dsc());
    }
}
