#![allow(dead_code)]
//! Imap - file for creating handling IMAP connection
//!
//! (C) 2015 Chris J Arges <christopherarges@gmail.com>
//!

extern crate imap;
extern crate openssl;
extern crate toml;

use self::openssl::ssl::{SslContext, SslMethod};
use self::imap::client::IMAPStream;
use self::imap::client::IMAPMailbox;

use conf::Conf;

pub struct Imap<'a> {
    conf: &'a Conf,
    socket: IMAPStream,
}

impl<'a> Imap<'a> {
  
    fn connect(conf: &'a Conf) -> IMAPStream {
        let imap_server = conf.get("accounts.default.imap_server").as_str().unwrap();
        let imap_port = conf.get("accounts.default.imap_port").as_integer().unwrap() as u16;
        let imap_ssl = Some(SslContext::new(SslMethod::Sslv23).unwrap());
        let imap_user = conf.get("accounts.default.imap_user").as_str().unwrap();
        let imap_pass = conf.get("accounts.default.imap_pass").as_str().unwrap();

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

    #[allow(unused_variables)]
    pub fn fetch_totals(&mut self, folder: &str) {
        match self.socket.select(folder) {
		Ok(IMAPMailbox{flags, exists, recent, unseen, permanent_flags, uid_next, uid_validity}) => {
			println!("exists: {}, recent: {}, unseen: {:?}", exists, recent, unseen);
		},
		Err(_) => println!("Error fetching totals from {}", folder)
       }
    }

    pub fn logout(&mut self) {
	if let Err(e) = self.socket.logout() {
		println!("Error logging out: {}", e)
	};
    }

    pub fn new(conf: &'a Conf) -> Imap {
        Imap { socket: Imap::connect(conf), conf: conf }
    }

}
