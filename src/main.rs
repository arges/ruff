/*
  (c) 2015 - Chris J Arges <christopherarges@gmail.com>
*/

extern crate ncurses;

mod screen;
use screen::*;

fn main() {
    let mut s = Screen::new();
    s.event_loop();	/* this should be run a thread */
    s.cleanup();
}
