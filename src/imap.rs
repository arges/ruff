#![allow(dead_code)]
//! Imap - file for creating handling IMAP connection
//!
//! (C) 2015 Chris J Arges <christopherarges@gmail.com>
//!

extern crate imap;
extern crate openssl;

use self::openssl::ssl::{SslContext, SslMethod};
use self::imap::client::IMAPStream;

/* TODO: plug Conf into imap */

pub struct Imap {
    socket: IMAPStream,
}

impl Imap {
  
    fn connect() -> IMAPStream {
        let imap_server = "";
        let imap_port = 993;
        let imap_ssl = Some(SslContext::new(SslMethod::Sslv23).unwrap());
        let imap_pass = "";
        let imap_user = "";

        let mut socket = match IMAPStream::connect(imap_server, imap_port,
            imap_ssl) {
            Ok(s) => s,
            Err(e) => panic!("Couldn't connect: {}", e)
        };

        match socket.login(imap_user, imap_pass) {
            Ok(_) => {},
            Err(e) => panic!("Couldn't login: {}", e)
        };

        socket
    }

    pub fn new() -> Imap {
        Imap { socket: Imap::connect() }
    }

}
