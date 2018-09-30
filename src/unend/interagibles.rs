use std::collections::HashMap;

/// These all the interactions than can be done with people or objects
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Interaction {
    Open,
    Close,
    Give,
    Take,
    Look,
    Push,
    Pull,
    Use,
}

lazy_static! {
    pub static ref INTERACTIONS: HashMap<&'static str, Interaction> = hashmap!{
        "open" => Interaction::Open,
        "close" => Interaction::Close,
        "give" => Interaction::Give,
        "take" => Interaction::Take,
        "look" => Interaction::Look,
        "push" => Interaction::Push,
        "pull" => Interaction::Pull,
        "use" => Interaction::Use,
    };
}

/// Possible results for an interaction
pub enum InteractionRes {
    Open,
    Close,
    Give,
    Take,
    Look(String),
    Push,
    Pull,
    Use,
}

/// Interagibles are objects and people
pub trait Interagible {
    fn get_tag(&self) -> String;
    fn get_name(&self) -> String;
    fn interact(&self, Interaction) -> InteractionRes;
}

#[derive(Debug, Clone)]
pub struct BasicObject {
    tag: String,
    name: String,
    dsc: String,  // We'll be providing this when player "looks" at the object
}

impl BasicObject {
    pub fn new(
        i_tag: String,
        i_name: String,
        i_dsc: String,
        // i_sdsc: String,
    ) -> Self {
        BasicObject {
            tag: i_tag,
            name: i_name,
            dsc: i_dsc,
        }
    }
}

impl Interagible for BasicObject {
    fn get_tag(&self) -> String {
        self.tag.clone()
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn interact(&self, iact: Interaction) -> InteractionRes {
        // Assume we can only look (for now :-))
        InteractionRes::Look(self.dsc.clone())
    }
}
