# The Undergarden

A text adventure game, with no death or dead ends.

Just for fun, and to learn [Rust](https://www.rust-lang.org/).

### TODO

Basically everything. :-)

The order of this list is relevant, but subject to change.

* Section navigation :white_check_mark:
* Objects/people the player can interact with (take, examine, ...). Objects and people are technically the same, as there is no reason not to be able to talk to an object; on the other side, people can be examined, pushed, etc
* Provide a way to win the game
* Compile to a _wasm_ target and have the game run in a web browser
* Non-obvious (i.e. more than one sentence) talk with people/objects
* Implement state in sections and objects: objects may change (i.e. a book is open or closed), and rooms may change their description and/or exits if the player does something
* Allow player to save and restore games
* Provide a way to load text data from a configuration fine
* Attempt to develop an alternative interface use _ncurses_ or so, in order to provide a semi-point&click experience by allowing to use buttons to move around, interact with objects/people and access to inventory
* I18N support

Months of fun ahead!!! :smirk:

### LICENSE

This work is distributed under the **MIT** license.
