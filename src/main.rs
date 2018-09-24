mod unend;
use unend::Visitable;
use std::collections::HashMap;


// use unend::Exitable;

// struct Hallway { section : unend::Section }
// struct Kitchen { section : unend::Section }

// impl unend::Exitable for Hallway {
//     fn n(&self) { println!("Let's go to the kitchen!"); } // We should return the tag of the room here
// }
// impl unend::Exitable for Kitchen {
//     fn n(&self) { println!("Outside, looking at the mountains."); } // We should return the tag of the room here
// }

// struct Candle {
//     dsc : String
// }

// impl unend::Interagible for Candle {
//     fn look(&self) {
//         println!("I am : {}", self.dsc);
//     }
// }



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

    println!("{}", hallway.tag());

    println!("Going N: {:?}", hallway.n());
    println!("Going S: {:?}", hallway.s());
    println!("Going W: {:?}", hallway.w());
    println!("Going E: {:?}", hallway.e());

    // let candle = Candle {
    //     dsc : "You are on the entrance".to_string(),
    // };

    //candle.look();
}
