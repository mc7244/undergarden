use std::collections::HashMap;
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
    fn n(&self) -> Exit { Exit::None }
    fn s(&self) -> Exit { Exit::None }
    fn w(&self) -> Exit { Exit::None }
    fn e(&self) -> Exit { Exit::None }
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
    start_section: String,
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

    fn n(&self) -> Exit {
        match self.exits.get("n") {
            Some(ex) => ex.clone(),
            None => Exit::None,
        }
    }
    fn s(&self) -> Exit {
        match self.exits.get("s") {
            Some(ex) => ex.clone(),
            None => Exit::None,
        }
    }
    fn w(&self) -> Exit {
        match self.exits.get("w") {
            Some(ex) => ex.clone(),
            None => Exit::None,
        }
    }
    fn e(&self) -> Exit {
        match self.exits.get("e") {
            Some(ex) => ex.clone(),
            None => Exit::None,
        }
    }
}

impl<T: Visitable> ConsoleGame<T> {
    pub fn new(sections: HashMap<String, T>, start_section : &str) -> Self {
        ConsoleGame {
            sections : sections,
            start_section : start_section.to_string(),
        }
    }

    pub fn run(&self) {
        let section = self.sections.get(&self.start_section).unwrap();
        println!("SECTION: {}", section.tag());

        let mut command = String::new();
        print!("> ");
        io::stdout().flush().unwrap(); // Or it won't print, as stdout is line-buffered

        match io::stdin().read_line(&mut command) {
            Ok(_)       => (),
            Err(error)  => panic!("Input error: {}", error),
        };

        let response = match ref command {
            "n" => section.n(),
            "s" => section.s(),
            "w" => section.w(),
            "e" => section.e(),
           // _   => "Unknown command!!"
        };

        println!("{:?}", response);
        
        // println!("Going N: {:?}", );
        // println!("Going S: {:?}", section.s());
        // println!("Going W: {:?}", section.w());
        // println!("Going E: {:?}", section.e());
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
