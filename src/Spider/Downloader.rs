use std::str;
use std::string::String;
use std::collections::hash_map::HashMap;
pub use std::io::{self, Write};
pub use futures::{Future, Stream};
pub use hyper::Client;
pub use tokio_core::reactor::Core;
use std::io::Read;



pub struct Downloader{
    conn: Client
}

impl Downloader{
    pub fn new ()  -> Downloader {
        let mut core = Core::new()?;
        let mut client = Client::new(&core.handle());
        return Downloader{conn: client};
    }

    pub fn fetch(&mut self,url: &mut String) -> String {
        let uri = url.parse()?;
        let mut body = String::new();
        let work = self.conn.get(uri).and_then(|res|{
            res.body().for_each(|chunk| {
                body.
            })
        });
        return body;
    }

   pub  fn to_file(&mut self,url: String,filepath: String){

    }

    pub fn to_dom(&mut self,url: String,filepath: String){
        
    }
}
