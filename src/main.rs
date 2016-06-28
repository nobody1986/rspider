extern crate hyper;
extern crate syncbox;
extern crate time;
extern crate url;
extern crate select;
//extern crate redis;
extern crate rusqlite;

pub mod Spider;

use std::str;
use std::string::String;
use std::collections::hash_map::HashMap;




fn main() {
    let mut s = Spider::Spider::Spider::new(String::from("http://www.csdn.net/"),3);
    s.addWhieteList(String::from("www.csdn.net"));
    s.addWhieteList(String::from("csdn.net"));
    s.run();
}
