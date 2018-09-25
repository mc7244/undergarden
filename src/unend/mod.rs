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

/// A basic section (`Visitable`), which can be instantiated by passing all descriptions
/// and exits as parameters. Only allows values for exits, we'll want to
/// implement a `Visitable` which accepts closures in the future.
pub struct BasicSection {
    tag: String,
    name: String,
    dsc: String,
    exits: HashMap<String, Exit>,
}

pub struct ConsoleGame<T: Visitable> {
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

impl<T: Visitable> ConsoleGame<T> {
    pub fn new(sections: HashMap<String, T>, start_section_tag : &str) -> Self {
        ConsoleGame {
            sections : sections,
            start_section_tag : start_section_tag.to_string(),
        }
    }

    pub fn run(&self) {
        let mut current_section_tag = self.start_section_tag.clone();

        loop {
            let current_section = self.sections.get(&current_section_tag).unwrap();
            println!("You are in the {}", current_section.name());

            // TODO: move input to a trait with a default implementation, so that
            // input can be actually changed.
            // Same for output!
            let mut player_input = String::new();
            print!("> ");
            io::stdout().flush().unwrap(); // Or it won't print, as stdout is line-buffered
            match io::stdin().read_line(&mut player_input) {
                Ok(_)       => (),
                Err(error)  => panic!("Input error: {}", error),
            };
            let command = player_input.to_lowercase().trim_right().to_string();

            match command.as_str() {
                dir if ( dir == "n" || dir == "s" || dir == "w" || dir == "e") => {
                    match current_section.exit(dir) {
                        Exit::Visitable(s) => {
                            current_section_tag = s;
                            continue;
                        },
                        Exit::Closed(s) => {
                            println!("{}", s);
                            continue;
                        },
                        Exit::None => {
                            println!("No exit this way.");
                            continue;
                        }
                    };
                }
                "q" => {
                    println!("See you!");
                    process::exit(0);
                }
                _   => {
                    println!("Invalid command.");
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
