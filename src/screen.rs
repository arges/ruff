//! Screen - handles screen input and output
//!
//! (C) 2015 Chris J Arges <christopherarges@gmail.com>
//!
//! # Example
//!
//! +--------------+
//! |FOLDERS|THREAD|
//! |       +------+
//! |       |MAIL  |
//! |       |      |
//! +-------+------+
//! |STATUS        |
//! +--------------+
//!
//! The Screen object is the curses interface, and the View objects are the
//! windows. Layout should be completely customizable.
//!
//! Typical session: <Tab> to highlight View, then <Space> to fullscreen View
//! <Enter> to select an item 'open folder/email/expand thread'
//! <hjkl/arrows> to move between items

extern crate ncurses;
use self::ncurses::*;

struct Item {
    text: String,
}

/* fix attrs in ncurses library they are wrong */
const B_BOLD: u64 = 1 << (NCURSES_ATTR_SHIFT + 13);

/* TODO remove extra fields in struct */
#[allow(dead_code)]
struct View {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    win: WINDOW,
    focus: bool,
    start: usize,
    index: usize,
    items: Vec<Item>
}

impl View {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> View {
        let win = newwin(height, width, y, x);
        box_(win, 0, 0);
	View { x: x, y: y, width: width, height: height, win: win,
               focus: false, start: 0, index: 0, items: Vec::new()
        }
    }

    /* TODO generalize color drawing code to hook in with Conf */
    fn get_color(focus: bool, highlight: i16) -> chtype {
        if focus {
            ' ' as chtype | COLOR_PAIR(highlight) as chtype | B_BOLD
        } else {
            ' ' as chtype | COLOR_PAIR(COLOR_PAIR_DEFAULT) as chtype
        }
    }

    pub fn redraw(&self) -> () {
        /* draw focus */
        wbkgd(self.win, View::get_color(self.focus, COLOR_PAIR_HIGHLIGHT));

        /* determine the total number of items to display */
        let items_max = if self.items.len() >= (self.height - 2) as usize {
            (self.height - 2) as usize
        } else {
            self.items.len()
        };

        /* draw each element */
        for i in self.start..(items_max + self.start) {
            let focus_color = View::get_color(self.index == i, COLOR_PAIR_FOCUS);
            wattron(self.win, focus_color as i32);
            mvwhline(self.win, i as i32 + 1 - self.start as i32, 1, focus_color as u64, self.width-2);
            mvwprintw(self.win, i as i32 + 1 - self.start as i32, 1, &self.items[i].text[..]);
            wattroff(self.win, focus_color as i32);
        }

        /* refresh */
        wrefresh(self.win);
    }

    /* FIXME scrolling isn't great */
    pub fn down(&mut self) {
        if self.index < self.items.len()-1 {
            self.index += 1;
        }
        self.start = if self.index > self.height as usize - 2 - 1 {
            self.index - (self.height as usize - 2 - 1)
        } else { 0 };
    }

    pub fn up(&mut self) {
        if self.index > 0 {
            self.index -=1;
        }
        self.start = if self.index > self.height as usize - 2 - 1 {
            self.index - (self.height as usize - 2 - 1)
        } else { 0 };
    }
}

#[allow(dead_code)]
pub struct Screen {
    width: i32,
    height: i32,
    index: usize,
    views: Vec<View>,
    status: String,
}

/* hack */
const KEY_H:i32 = 'h' as i32;
const KEY_J:i32 = 'j' as i32;
const KEY_K:i32 = 'k' as i32;
const KEY_L:i32 = 'l' as i32;
const KEY_TAB:i32 = '\t' as i32;

/* colors */
static COLOR_PAIR_DEFAULT: i16 = 1;
static COLOR_PAIR_HIGHLIGHT: i16 = 2;
static COLOR_PAIR_FOCUS: i16 = 3;

/* strings */
static INITIAL_STATUS: &'static str = "ruff email - F1 to exit; TAB to cycle Views, hjkl/arrow keys to navigate Items";

/* TODO error handling for ncurses */
impl Screen {
    pub fn new() -> Screen {
        /* initialize the screen */
        initscr();
        raw();
        keypad(stdscr, true);
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
	refresh();

	/* setup colors */
        start_color();
        use_default_colors();
        init_pair(COLOR_PAIR_DEFAULT, COLOR_WHITE, -1);
        init_pair(COLOR_PAIR_HIGHLIGHT, COLOR_YELLOW, -1);
        init_pair(COLOR_PAIR_FOCUS, COLOR_YELLOW, COLOR_BLUE);

        /* get screen boundaries */
        let mut height = 0;
        let mut width = 0;
        getmaxyx(stdscr, &mut height, &mut width);

        /* create views TODO make this configurable */
        let mut folders = View::new(1,1, width/4 - 1, height - 2);
        let mut threads = View::new(width/4 - 1, 1, 3*(width/4) - 1, height - 2);
        threads.focus = true;

        /* FIXME: insert demo items */
        for i in 1..50 {
            let folder_string = format!("Folder {}", i);
            folders.items.push(Item{text: folder_string.to_string()});
            let item_string = format!("Subject {}", i);
            threads.items.push(Item{text: item_string.to_string()});
        }
	let mut views = Vec::new();
	views.push(folders);
	views.push(threads);

        /* return screen object with default view highlighted */
        Screen { width: width, height: height, index: 0, views: views, status: INITIAL_STATUS.to_string() }
    }

    pub fn resize(&mut self) {
        /* get screen boundaries */
        getmaxyx(stdscr, &mut self.height, &mut self.width);

        /* resize all views */
    }

    pub fn update(&mut self) -> () {
        /* determine who has current focus only update things on screen */
        for i in 0..self.views.len() {
            self.views[i].focus = false;
        }
        self.views[self.index].focus = true;
    }

    pub fn redraw(&self) -> () {
        /* redraw all views */
        for view in &self.views {
            view.redraw();
        }

        /* print status */
        mvprintw(self.height-1,0, &self.status[..]);
        refresh();
    }

    pub fn event_loop(&mut self) -> () {
        &self.update();
        &self.redraw();
        let mut ch = getch();

        while ch != KEY_F(1)
        {
            /* get input */
            match ch
            {
                KEY_TAB => {
                    self.index = (self.index + 1) % self.views.len();
                },
                KEY_H | KEY_LEFT => { self.index = if self.index > 0 {
                    (self.index - 1) % self.views.len() } else { 0 };
                },
                KEY_J | KEY_DOWN => { self.views[self.index].down(); },
                KEY_K | KEY_UP => { self.views[self.index].up(); },
                KEY_L | KEY_RIGHT => {
                    self.index = if self.index < self.views.len() - 1 {
                        (self.index + 1) % self.views.len()
                    } else { self.views.len() - 1 }; },
                KEY_RESIZE => {
                    self.resize();
                }
                _ => { },
            }
            &self.update();
            &self.redraw();

            ch = getch();
        }
    }

    pub fn cleanup(&self) -> () {
        endwin();
    }
}
