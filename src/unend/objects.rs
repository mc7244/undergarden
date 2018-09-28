/// These all the interactions than can be done with people or objects
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Interactions {
    Open,
    Close,
    Give,
    Take,
    Look,
    Push,
    Pull,
    Use,
}

/// Interagibles are objects and people
pub trait Interagible {
    fn get_tag(&self) -> String;
    fn get_name(&self) -> String;
    fn interact(&self) {}
    // fn interact_with(&self, &Interagible) {}
}
