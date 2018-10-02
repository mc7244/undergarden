use std::collections::HashMap;

pub enum UnendObject {
    Info(InfoObject),
    Portal(PortalObject),
}

/// These all the interactions than can be done with people or objects
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Interaction {
    Open,
    Close,
    Give,
    Take,
    Look,
    Talk,
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
        "talk" => Interaction::Talk,
        "push" => Interaction::Push,
        "pull" => Interaction::Pull,
        "use" => Interaction::Use,
    };
}

/// Possible results for an interaction
pub enum InteractionRes {
    Info(String),
    GotoSection(String),
}

/// Interagibles are objects and people
pub trait Interagible {
    fn get_tag(&self) -> String;
    fn get_name(&self) -> String;
    fn interact(&self, Interaction) -> InteractionRes;
}

/// An object which can only give information (=return strings) when
/// interacted with.
#[derive(Debug, Clone)]
pub struct InfoObject {
    tag: String,
    name: String,
    av_interactions: HashMap<Interaction, String>,
}

/// An object which can only give information (=return strings) when
/// interacted with.
#[derive(Debug, Clone)]
pub struct PortalObject {
    tag: String,
    name: String,
    dsc: String,
    destination: String,
}

impl InfoObject {
    pub fn new(
        i_tag: String,
        i_name: String,
        i_av_interactions: HashMap<Interaction, String>
    ) -> Self {
        InfoObject {
            tag: i_tag,
            name: i_name,
            av_interactions: i_av_interactions
        }
    }
}

impl Interagible for InfoObject {
    fn get_tag(&self) -> String {
        self.tag.clone()
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn interact(&self, iact: Interaction) -> InteractionRes {
        // See if we have a string available for this interaction
        match self.av_interactions.get(&iact) {
            Some(s) => InteractionRes::Info(s.to_string()),
            None => InteractionRes::Info("That won't work".to_string())
        }
    }
}

impl PortalObject {
    pub fn new(
        i_tag: String,
        i_name: String,
        i_dsc: String,
        i_destination: String,
    ) -> Self {
       PortalObject {
            tag: i_tag,
            name: i_name,
            dsc: i_dsc,
            destination: i_destination
        }
    }
}

impl Interagible for PortalObject {
    fn get_tag(&self) -> String {
        self.tag.clone()
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn interact(&self, iact: Interaction) -> InteractionRes {
        // See if we have a string available for this interaction
        match iact {
            Interaction::Look => InteractionRes::Info(self.dsc.clone()),
            Interaction::Use => InteractionRes::GotoSection(self.destination.clone()),
            _ => InteractionRes::Info("That won't work".to_string()),
        }
    }
}