/*
  (c) 2015 - Chris J Arges <christopherarges@gmail.com>
*/

/* this file may be renamed */

extern crate ncurses;
use self::ncurses::*;

/*

Example screen layout:
+--------------+
|FOLDERS|THREAD|
|       +------+
|       |MAIL  |
|       |      |
+-------+------+
|STATUS        |
+--------------+

The Screen object is the curses interface, and the View objects are the
windows. Layout should be completely customizable.

*/

/* fix attrs in ncurses library they are wrong */
const B_BOLD: u64 = 1 << (NCURSES_ATTR_SHIFT + 13);

#[allow(dead_code)]
struct View {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    win: WINDOW,
    focus: bool,
    list: Vec<String>
}

impl View {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> View {
        let win = newwin(height, width, y, x);
        box_(win, 0, 0);
        wrefresh(win);
        View { x: x, y: y, width: width, height: height, win: win, focus: false, list: Vec::new()}
    }

    pub fn focus(&mut self) -> () {
        self.focus = true;
        wbkgd(self.win, ' ' as chtype | COLOR_PAIR(COLOR_PAIR_HIGHLIGHT) as chtype | B_BOLD as u64);
        wrefresh(self.win);
    }

    pub fn unfocus(&mut self) -> () {
        self.focus = false;
        wbkgd(self.win, ' ' as chtype | COLOR_PAIR(COLOR_PAIR_DEFAULT) as chtype);
        wrefresh(self.win);
    }
}

#[allow(dead_code)]
pub struct Screen {
    width: i32,
    height: i32,
    index: usize,
    views: Vec<View>,
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

        /* get screen boundaries */
        let mut height = 0;
        let mut width = 0;
        getmaxyx(stdscr, &mut height, &mut width);

        /* create views */
        let mut folders = View::new(1,1, width/4 - 1, height - 2);
        let threads = View::new(width/4 - 1, 1, 3*(width/4) - 1, height/2 + 1);
        let messages = View::new(width/4 - 1,height/2 + 1, 3*(width/4) - 1, height/2 - 2);
        folders.focus();
	let mut views = Vec::new();
	views.push(folders);
	views.push(threads);
	views.push(messages);

	/* setup status */
        mvprintw(height-1,0, "ruff email - F1 to exit; TAB to navigate.");

        /* return screen object with default view highlighted */
        Screen { width: width, height: height, index: 0, views: views}
    }

    pub fn event_loop(&mut self) -> () {
        let mut ch = getch();
        while ch != KEY_F(1)
        {
            /* get input */
            match ch
            {
                KEY_TAB => { self.index = (self.index + 1) % 3; },
                KEY_H => { },
                KEY_J => { },
                KEY_K => { },
                KEY_L => { },
                _ => { },
            }

            /* update windows */
            for i in 0..3 {
                self.views[i].unfocus();
            }
            self.views[self.index].focus();

            refresh();
            ch = getch();
        }
    }

    pub fn cleanup(&self) -> () {
        endwin();
    }
}
