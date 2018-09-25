mod unend;
use std::collections::HashMap;

fn main() {
    let mut hallway_exits = HashMap::new();
    hallway_exits.insert("n".to_string(), unend::Exit::Visitable("kitchen".to_string()));
    hallway_exits.insert("e".to_string(), unend::Exit::Closed("It's not time for gardening.".to_string()));
    let hallway = unend::BasicSection::new(
        "kitchen".to_string(),
        "Main hallway".to_string(),
        "You are on the main hallway. You can go north to the kitchen, south to the sitting room, ...".to_string(),
        hallway_exits
        // exits: unend::ClosureExits {
        //     n : || { println!("Going to the kitchen"); },
        // },
    );

    let mut kitchen_exits = HashMap::new();
    kitchen_exits.insert("s".to_string(), unend::Exit::Visitable("hallway".to_string()));
    kitchen_exits.insert("e".to_string(), unend::Exit::Closed("You can't exit through the window.".to_string()));
    let kitchen = unend::BasicSection::new(
        "kitchen".to_string(),
        "The grand kitchen".to_string(),
        "You are at the center of the kitchen and dining room.".to_string(),
        kitchen_exits
    );

    let mut sections = HashMap::new();
    sections.insert("hallway".to_string(), hallway);
    sections.insert("kitchen".to_string(), kitchen);

    let game = unend::ConsoleGame::new(sections, "hallway");

    game.run();
}
