mod unend;

use std::collections::HashMap;
use unend::{ConsoleIO, Game};
use unend::visitables::{BasicSection, Exit, ExitDir, Visitable};

#[macro_use]
extern crate maplit;

#[macro_use]
extern crate lazy_static;

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
    );

    let kitchen = BasicSection::new(
        "kitchen".to_string(),
        "The grand kitchen".to_string(),
        "You are at the center of the kitchen and dining room.".to_string(),
        hashmap!{
            ExitDir::South => Exit::Visitable("hallway".to_string()),
            ExitDir::East => Exit::Closed("You can't exit through the window.".to_string()),
        },
    );

    hashmap!{
        hallway.get_tag() => hallway,
        kitchen.get_tag() => kitchen,
    }
}
