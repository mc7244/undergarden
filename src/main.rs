mod unend;

use std::collections::HashMap;
use unend::interagibles::{InfoObject, Interaction, PortalObject, UnendObject};
use unend::visitables::{BasicSection, Exit, ExitDir, UnendSection};
use unend::{ConsoleIO, Game};

#[macro_use]
extern crate maplit;
#[macro_use]
extern crate lazy_static;
extern crate regex;

// Since we have to create a lot of string in defining the game, we
// define a handy macro for that.
macro_rules! s {
    ($x:expr) => {
        String::from($x)
    };
}

// Our games does I/O from the console, so we implement the relative trait
// No need to actually implement methods, the default ones will do
impl ConsoleIO for Game {}

fn main() {
    let sections = create_sections();

    let initial_inventory = hashmap!{s!("busticket") => UnendObject::Info(InfoObject::new(
        s!("busticket"),
        s!("Used bus ticket"),
        hashmap!{
            Interaction::Look => s!("It's the ticket I used to come here. I can't use it anymore, at least not legally."),
        },
    ))};
    let mut game = Game::new(sections, "hallway".to_string(), initial_inventory);

    game.run();
}

/// Create the sections (we use `BasicSection`, which implements `Visitable`)
/// and return an HashMap containing them
fn create_sections() -> HashMap<String, UnendSection> {
    let hallway = UnendSection::Basic(BasicSection::new(
        s!("hallway"),
        s!("Main hallway"),
        s!("You can go north to the kitchen, south to the sitting room. There's a *fireplace* in the middle."),
        hashmap!{
            ExitDir::North => Exit::Visitable(s!("kitchen")),
            ExitDir::East => Exit::Closed(s!("This is no time for gardening.")),
        },
        hashmap!{s!("fireplace") => UnendObject::Portal(PortalObject::new(
            s!("fireplace"),
            s!("A strange fireplace"),
            s!("This fireplace glows like it's enchanted."),
            s!("secretroom"),
        ))},
    ));

    let kitchen = UnendSection::Basic(BasicSection::new(
        s!("kitchen"),
        s!("The grand kitchen"),
        s!("You are at the center of the kitchen and dining room. The only exit is south. There's a *book* on the table."),
        hashmap!{
            ExitDir::South => Exit::Visitable(s!("hallway")),
            ExitDir::East => Exit::Closed(s!("You can't exit through the window.")),
        },
        hashmap!{s!("book") => UnendObject::Info(InfoObject::new(
            s!("book"),
            s!("Pink Book"),
            hashmap!{
                Interaction::Look => s!("It is a strange pink book with a black sheep on the cover."),
                Interaction::Take => s!("I don't need this book."),
                Interaction::Use  => s!("Hmmm, I prefer to watch movies rather than read."),
            },
        ))},
    ));

    let secretroom = UnendSection::Basic(BasicSection::new(
        s!("secretroom"),
        s!("Secret Room"),
        s!("A very strange and dark secret room. You see a light east, maybe an exit"),
        hashmap!{
            ExitDir::East => Exit::Visitable(s!("hallway")),
        },
        hashmap!{},
    ));

    hashmap!{
        s!("hallway") => hallway,
        s!("kitchen") => kitchen,
        s!("secretroom") => secretroom,
    }
}
