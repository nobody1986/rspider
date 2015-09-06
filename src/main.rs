
extern crate hyper;
extern crate syncbox;
extern crate time;
extern crate url;
extern crate htmlstream;
extern crate redis;

pub mod Spider;

use std::str;
use std::string::String;
use std::collections::hash_map::HashMap;




fn main() {
    let mut s = Spider::Spider::Spider::new(String::from("http://www.oschina.net/"),3);
    s.addWhieteList(String::from("www.oschina.net"));
    s.addWhieteList(String::from("oschina.net"));
    s.run();
}