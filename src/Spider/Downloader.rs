use std::str;
use std::string::String;
use std::collections::hash_map::HashMap;
pub use hyper::Client;
pub use hyper::header::Connection;
use std::io::Read;



pub struct Downloader{
    conn: Client
}

impl Downloader{
    pub fn new ()  -> Downloader {
        let mut client = Client::new();
            return Downloader{conn: client};
    }

    pub fn fetch(&mut self,url: &mut String) -> String {
        let mut res = self.conn.get(&(*url))
            .header(Connection::close())
            .send().unwrap();

        let mut body = String::new();
         match res.read_to_string(&mut body) {
            Ok(i) => (),
            Err(_) => ()
         }

         return body;
    }

   pub  fn to_file(&mut self,url: String,filepath: String){

    }

    pub fn to_dom(&mut self,url: String,filepath: String){
        
    }
}
