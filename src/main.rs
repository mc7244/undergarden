mod unend;
use unend::ConsoleIO;

#[macro_use] extern crate maplit;

// Our games does I/O from the console, so we implement the relative trait
// No need to actually implement methods, the default ones will do
impl<T: unend::Visitable> ConsoleIO for unend::Game<T> {}

fn main() {
    let hallway = unend::BasicSection::new(
        "kitchen".to_string(),
        "Main hallway".to_string(),
        "You are on the main hallway. You can go north to the kitchen, south to the sitting room, ...".to_string(),
        hashmap!{
            "n".to_string() => unend::Exit::Visitable("kitchen".to_string()),
            "e".to_string() => unend::Exit::Closed("There's no time for gardening.".to_string()),
        }
    );

    let kitchen = unend::BasicSection::new(
        "kitchen".to_string(),
        "The grand kitchen".to_string(),
        "You are at the center of the kitchen and dining room.".to_string(),
        hashmap!{
            "s".to_string() => unend::Exit::Visitable("hallway".to_string()),
            "e".to_string() => unend::Exit::Closed("You can't exit through the window.".to_string()),
        }
    );

    let sections = hashmap!{
        "hallway".into() => hallway,
        "kitchen".into() => kitchen,
    };

    let game = unend::Game::new(sections, "hallway");

    game.run();
}
