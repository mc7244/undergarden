use std::collections::HashMap;
use std::process;
use std::io::{self, Write};

/// Exit from each section. Can be the tag of any other `Visitable`, so exotic
/// things such as portals are indeed supported
#[derive(Debug, Clone)]
pub enum Exit {
    Visitable(String),
    Closed(String),
    None
}

/// Anything that is `Visitable` can be a section in the game.
/// Default implementation (no exit) is provided for all exits, so only existing
/// ones needs to be explicitly implemented
pub trait Visitable {
    fn tag(&self) -> String;
    fn name(&self) -> String;
    fn dsc(&self) -> String;
    fn exit(&self, _dir: &str) -> Exit { Exit::None }
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
            Ok(_)       => (),
            Err(error)  => panic!("Input error: {}", error),
        };
        player_input.to_lowercase().trim().to_string()
    }
    /// Write the passed string, adding a _newline_
    fn write_line(&self, line: &str) {
        println!("{}", line);
    }
}

/// A basic section (`Visitable`), which can be instantiated by passing all descriptions
/// and exits as parameters. Only allows values for exits, we'll want to
/// implement a `Visitable` which accepts closures in the future.
pub struct BasicSection {
    tag: String,
    name: String,
    dsc: String,
    exits: HashMap<String, Exit>,
}

pub struct Game<T: Visitable> {
    sections: HashMap<String, T>,
    start_section_tag: String,
}

impl BasicSection {
    pub fn new(tag: String, name: String, dsc: String, exits: HashMap<String, Exit>) -> Self {
        BasicSection {
            tag: tag, name: name, dsc: dsc, exits: exits
        }
    }
}

impl Visitable for BasicSection {
    // TODO think: Would it be enough to return a reference instead of cloning?
    fn tag(&self) -> String { self.tag.clone() }
    fn name(&self) -> String { self.name.clone() }
    fn dsc(&self) -> String { self.dsc.clone() }

    fn exit(&self, dir: &str) -> Exit {
        match self.exits.get(dir) {
            Some(ex) => ex.clone(),
            None => Exit::None,
        }
    }
}

impl<T: Visitable> Game<T> {
    pub fn new(sections: HashMap<String, T>, start_section_tag : &str) -> Self {
        Game {
            sections : sections,
            start_section_tag : start_section_tag.to_string(),
        }
    }

    pub fn run(&self) {
        let mut current_section_tag = self.start_section_tag.clone();

        loop {
            let current_section = self.sections.get(&current_section_tag).unwrap();
            self.write_line(&format!("You are in the {}", current_section.name()));

            let command = self.read_command();

            match command.as_str() {
                dir if ( dir == "n" || dir == "s" || dir == "w" || dir == "e") => {
                    match current_section.exit(dir) {
                        Exit::Visitable(s) => {
                            current_section_tag = s;
                            continue;
                        },
                        Exit::Closed(s) => {
                            self.write_line(&s);
                            continue;
                        },
                        Exit::None => {
                            self.write_line("No exit this way.");
                            continue;
                        }
                    };
                }
                "q" => {
                    self.write_line("See you!");
                    process::exit(0);
                }
                _   => {
                    self.write_line("Invalid command.");
                    continue;
                }
            };
        }
    }
}

// Interagibles are objects and people
// pub trait Interagible {
//     fn open(&self) { println!("Open!"); }
//     fn close(&self) { println!("Close!"); }
//     fn give(&self) { println!("Give!"); }
//     fn take(&self) { println!("Take!"); }
//     fn look(&self) { println!("Look!"); }
//     fn talk(&self) { println!("Talk!"); }
//     fn push(&self) { println!("Push!"); }
//     fn pull(&self) { println!("Pull!"); }
//     fn utilize(&self) { println!("Use!"); }
// }

// pub struct Element {
//     pub tag : String,
//     pub name : String,
//     pub dsc : String,
// }
