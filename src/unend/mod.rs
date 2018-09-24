use std::collections::HashMap;

/// Exists from each section. Can be the tag of any other `Visitable`, so exotic
/// things such as portals are indeed supported
#[derive(Debug, Clone)]
pub enum Exit {
    Visitable(String),
    Closed(String),
    None
}

pub type Exits = HashMap<String, Exit>;

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

pub struct BasicSection {
    tag: String,
    name: String,
    dsc: String,
    exits: Exits,
}

// TODO: pass HashMap with dir->tag to implement basic exits
impl BasicSection {
    pub fn new(tag: String, name: String, dsc: String, exits: Exits) -> Self {
        BasicSection {
            tag: tag, name: name, dsc: dsc, exits: exits
        }
    }
}

impl Visitable for BasicSection {
    // FIXME: Would it be enough to return a reference instead of cloning?
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

// pub struct Exits
// {
//     pub n : Fn() -> ()
// }

// /// An Element can be an object or a person
// pub struct Element {
//     pub tag : String,
//     pub name : String,
//     pub dsc : String,
// }

// pub struct Section
// {
//     pub tag : String,
//     pub name : String,
//     pub dsc : String,
//     pub exits : Exits,
// }

// impl Section {
//     pub fn new(section : Section) -> Self {
//         Section(section)
//     }
//     pub fn exit(&self) {
//         (self.exits.n)();
//     }
// }

// pub fn info() -> String {
//     "This is UnEnd 0.0.1".to_string()
// }