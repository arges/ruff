//! Ruff command line e-mail client
//!
//! (C) 2015 Chris J Arges <christopherarges@gmail.com>
//!
extern crate ncurses;

mod screen;
mod conf;

use screen::Screen;
use conf::Conf;

#[allow(unused_variables)]
fn main() {
    let mut s = Screen::new();
    let c = Conf::new();

    s.event_loop();
    s.cleanup();
}
