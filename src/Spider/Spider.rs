use syncbox::{ThreadPool, Run};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::*;
use std::str;
use std::string::String;
use std::collections::hash_map::HashMap;
use url::{Url,Host,ParseError};

use Spider::MessageQueue;
use Spider::HtmlParser;
use Spider::Downloader;

pub struct Spider{
    seed: String,
    level: i32,
    down: Downloader::Downloader,
    queue: MessageQueue::MessageQueue,
    whilelist: HashMap<String,i32>,
    domain: String
}

impl Spider{
    pub fn new (seed: String,level: i32)  -> Spider {
        let url_parsed = Url::parse(&seed);
        let d = match url_parsed {
                    Ok(u) => String::from(u.domain().expect("none")),
                    Err(_) => String::from("")
                };
            return Spider{
                whilelist: HashMap::new(),
                down: Downloader::Downloader::new(),
                queue: MessageQueue::MessageQueue::new(),
                seed:seed,
                level: level,
                domain: d
            };
    }

    pub fn addWhieteList(&mut self,domain: String){
    	self.whilelist.insert(domain,1);
    }

    pub fn isInWhiteList(&mut self,url:&mut String) -> bool {
        //println!("{:?}",url );
        return self.whilelist.contains_key(url);
    }

    pub fn mkFullUrl(&mut self,url: &mut String) -> String{
        let url_parsed = Url::parse(&url);
        let mut domain: String  =  match url_parsed {
            Ok(u) =>   match u.domain() {
                Some(i) => i.to_string(),
                None => String::from("")
            },
            Err(y) => String::from("")
        };
        //println!("{:?}",domain );
        if domain == ""{
            return String::from("");
        }
        let seed_parsed = Url::parse(&(self.seed)).unwrap();
        //let mut full_url_string:String = match UrlParser::new().base_url(&seed_parsed).parse(&url) {
        //    Ok(u) => u.serialize(),
        //    Err(y) => String::from("")
        //};

        //println!("{:?}", full_url_string);
        let full_url_string = url.clone();
        if self.isInWhiteList(&mut domain) {
            return full_url_string;
        }
        return String::from("");
    }

    pub fn work(&mut self,url: &mut String){
        println!("{:?}",url);
        let level = self.queue.url_exists(url);
        //println!("{:?}",level);
        if level > 0 {
            if url != "" {
                let mut s = self.down.fetch( url);
                let mut p = HtmlParser::HtmlParser::new(s);
                let mut r = p.get_a();
                for i in 0..r.len() {
//println!("{:?}",r[i]);
                    let mut u = self.mkFullUrl(&mut r[i]);
                    //println!("{:?}",u);
                    if u != "" {
                        self.queue.add_to_queue(&mut u,level - 1);
                    }
                }
            }
        }


    }

    pub fn run( &mut self)   {
        self.queue.add_to_queue(&mut self.seed,self.level);
        let sign = 1;
        while sign > 0 {
            let mut url = self.queue.get_url();
            println!("url:{:?}", url);
            if url == "" {
                break;
            }
            self.work(&mut url);
        }
    }
}
