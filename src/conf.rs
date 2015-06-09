/* configuration code

use toml format since it is pretty standard in rust

[colors]
  theme = dark

[keys]
  style = vim

[account]
  [account.name]
    imap_user = "user@host.com"
    smtp_url = ""
*/

extern crate toml;
#[allow(unused_imports)]
use self::toml::*;

#[allow(dead_code)]
const CONF_PATH:&'static str= "~/.ruffrc";

pub struct Conf;

impl Conf {
    pub fn new() -> Conf {
        Conf
    }
}
