mod unend;
//use unend::Visitable;
use std::collections::HashMap;

fn main() {
    let mut exits = HashMap::new();
    exits.insert("n".to_string(), unend::Exit::Visitable("kitchen".to_string()));
    exits.insert("e".to_string(), unend::Exit::Closed("It's not time for gardening!".to_string()));

    let hallway = unend::BasicSection::new(
        "hallway".to_string(),
        "Main hallway".to_string(),
        "You are on the main hallway. You can go north to the kitchen, south to the sitting room, ...".to_string(),
        exits
        // exits: unend::ClosureExits {
        //     n : || { println!("Going to the kitchen"); },
        // },
    );

    let mut sections = HashMap::new();
    sections.insert("hallway".to_string(), hallway);

    let game = unend::ConsoleGame::new(sections, "hallway");

    game.run();
}
