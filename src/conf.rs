#![allow(dead_code)]
//! Conf - configuration file loading and parsing
//!
//! (C) 2015 Chris J Arges <christopherarges@gmail.com>
//!
//! # Examples
//! ```toml
//! [theme]
//!  colors = dark
//!
//! [keys]
//!  style = vim
//!
//! [accounts]
//!   [accounts.default]
//!     imap_server = "imap.example.com"
//!     imap_port = "993"
//!     imap_user = "user@example.com"
//!     imap_pass = ""
//!
//! ```

extern crate toml;
use self::toml::*;
use std::fs::File;
use std::io::prelude::*;
use std::env;

const CONF_FILE:&'static str= ".ruffrc";

pub struct Conf {
    conf: toml::Value,
}

impl Conf {
    /// Loads a new configuration file
    fn load() -> toml::Value {
        /* load the configuration file */
        let path = env::home_dir().unwrap().join(CONF_FILE);
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(..) => { panic!("Configuration file: {:?} not found.", path); }
        };
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Ok(..) => {}
            Err(..) => { panic!("Unable to read configuration file: {:?}.", path); }
        }

        /* parse the string and nicely print any errors */
        let mut parser = toml::Parser::new(&s[..]);
        let toml = match parser.parse() {
            Some(value) => value,
            None => {
                for err in &parser.errors {
                    let (loline, locol) = parser.to_linecol(err.lo);
                    let (hiline, hicol) = parser.to_linecol(err.hi);
                    println!("{:?}:{}:{}-{}:{} error: {:?}",
                             path, loline, locol, hiline, hicol, err.desc);
               }
               panic!("Malformed Configuration File {:?}", path); }
        };

        /* return parsed configuration */
        Value::Table(toml)
    }

    /// Loads a configuration and constructions a new Conf object
    pub fn new() -> Conf {
        Conf { conf: Conf::load() }
    }

    /// Reloads a new configuration into Conf object
    pub fn reload(&mut self) {
        self.conf = Conf::load();
    }

    /// Lookup key from Conf object
    pub fn lookup(&self, key: &'static str) -> &toml::Value {
        self.conf.lookup(key).unwrap()
    }
}
