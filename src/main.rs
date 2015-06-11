#![allow(unused_imports)]
//! Ruff command line e-mail client
//!
//! (C) 2015 Chris J Arges <christopherarges@gmail.com>
//!
extern crate ncurses;

mod screen;
use screen::Screen;
mod conf;
use conf::Conf;
mod imap;
use imap::Imap;

#[allow(unused_variables)]
fn main() {
    let mut s = Screen::new();
    let c = Conf::new();

    s.event_loop();
    s.cleanup();
}
