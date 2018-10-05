use std::collections::HashMap;
use unend::interagibles::*;

pub enum UnendSection {
    Basic(BasicSection),
}

/// Exit from each section. Can be the tag of any other `Visitable`, so exotic
/// things such as portals are indeed supported. Or it can `Closed` or not existing.
#[derive(Debug, Clone)]
pub enum Exit {
    Visitable(String),
    Closed(String),
    None,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ExitDir {
    North,
    South,
    West,
    East,
}

lazy_static! {
    pub static ref EXITDIRS: HashMap<&'static str, ExitDir> = hashmap!{
        "n" => ExitDir::North,
        "s" => ExitDir::South,
        "w" => ExitDir::West,
        "e" => ExitDir::East,
    };
}

/// Anything that is `Visitable` can be a section in the game.
pub trait Visitable {
    fn get_tag(&self) -> String;
    fn get_name(&self) -> String;
    fn get_dsc(&self) -> String;
    fn get_objects(&self) -> &HashMap<String, UnendObject>;
    fn exit(&self, _dir: &ExitDir) -> Exit;
}

/// A basic section (`Visitable`), which can be instantiated by passing all descriptions
/// and exits as parameters. Only allows values for exits, we'll want to
/// implement a `Visitable` which accepts closures in the future.
pub struct BasicSection {
    tag: String,
    name: String,
    dsc: String,
    exits: HashMap<ExitDir, Exit>,
    pub objects: HashMap<String, UnendObject>,
}

impl BasicSection {
    pub fn new(
        i_tag: String,
        i_name: String,
        i_dsc: String,
        i_exits: HashMap<ExitDir, Exit>,
        i_objects: HashMap<String, UnendObject>,
    ) -> Self {
        BasicSection {
            tag: i_tag,
            name: i_name,
            dsc: i_dsc,
            exits: i_exits,
            objects: i_objects,
        }
    }
}

impl Visitable for BasicSection {
    fn get_tag(&self) -> String {
        self.tag.clone()
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_dsc(&self) -> String {
        self.dsc.clone()
    }
    fn get_objects(&self) -> &HashMap<String, UnendObject> {
        &self.objects
    }

    fn exit(&self, dir: &ExitDir) -> Exit {
        match self.exits.get(dir) {
            Some(ex) => ex.clone(),
            None => Exit::None,
        }
    }
}
