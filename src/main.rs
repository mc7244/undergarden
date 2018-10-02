mod unend;

use std::collections::HashMap;
use unend::{ConsoleIO, Game};
use unend::visitables::{BasicSection, Exit, ExitDir, Visitable};
use unend::interagibles::{UnendObject, InfoObject, PortalObject, Interaction, Interagible};

#[macro_use]
extern crate maplit;
#[macro_use]
extern crate lazy_static;
extern crate regex;

// Our games does I/O from the console, so we implement the relative trait
// No need to actually implement methods, the default ones will do
impl<T: Visitable> ConsoleIO for Game<T> {}

fn main() {
    let sections = create_sections();

    let mut game = Game::new(sections, "hallway".to_string());

    game.run();
}

/// Create the sections (we use `BasicSection`, which implements `Visitable`)
/// and return an HashMap containing them
fn create_sections() -> HashMap<String, BasicSection> {
    let hallway = BasicSection::new(
        "hallway".to_string(),
        "Main hallway".to_string(),
        "You can go north to the kitchen, south to the sitting room, ...".to_string(),
        hashmap!{
            ExitDir::North => Exit::Visitable("kitchen".to_string()),
            ExitDir::East => Exit::Closed("There's no time for gardening.".to_string()),
        },
        hashmap!{"fireplace".to_string() => UnendObject::Portal(PortalObject::new(
            "fireplace".to_string(),
            "A strange fireplace".to_string(),
            "This fireplace glows like it's enchanted.".to_string(),
            "secretroom".to_string(),
        ))},
    );

    let kitchen = BasicSection::new(
        "kitchen".to_string(),
        "The grand kitchen".to_string(),
        "You are at the center of the kitchen and dining room.".to_string(),
        hashmap!{
            ExitDir::South => Exit::Visitable("hallway".to_string()),
            ExitDir::East => Exit::Closed("You can't exit through the window.".to_string()),
        },
        hashmap!{"book".to_string() => UnendObject::Info(InfoObject::new(
            "book".to_string(),
            "Pink Book".to_string(),
            hashmap!{
                Interaction::Look =>  "It is a strange pink book with a black sheep on the cover.".to_string(),
                Interaction::Take =>  "I don't need this book.".to_string(),
            },
       ))},
    );

    let secretroom = BasicSection::new(
        "secretroom".to_string(),
        "Secret Room".to_string(),
        "A very strange and dark secret room. You see a light east, maybe an exit".to_string(),
        hashmap!{
            ExitDir::South => Exit::Visitable("hallway".to_string()),
        },
        hashmap!{},
    );

    hashmap!{
        hallway.get_tag() => hallway,
        kitchen.get_tag() => kitchen,
        secretroom.get_tag() => secretroom,
    }
}
