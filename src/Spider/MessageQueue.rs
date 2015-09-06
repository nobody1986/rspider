use redis;
use redis::{Commands, PipelineCommands, transaction};

use std::str;
use std::string::String;
use std::collections::hash_map::HashMap;


pub struct MessageQueue{
    conn: redis::Connection
}

impl MessageQueue{
    pub fn new ()  ->MessageQueue {
        let client =  match redis::Client::open("redis://127.0.0.1/") {
            Ok(x) => x,
            Err(_) => panic!("redis connect error") 
        };
        
            let con = match client.get_connection() {
                Ok(x) => x,
            Err(_) => panic!("redis connect error") 
            };
            return MessageQueue{conn: con};

    }

    pub fn add_to_queue (&mut self ,url: &mut String,level: i32){
        if self.url_exists(url) < 0 {
            let ret:Result<i32,_> =  self.conn.hset("visited",url.clone(),level );
            let ret:Result<String,_> = self.conn.lpush("url_queue",url.clone());
        }
    }

    pub fn url_exists (&mut self ,url:  &mut  String) -> i32{
        let ret:Result<i32,_> = self.conn.hget("visited",url.clone());
        //println!("{:?}",ret );
        return match ret {
            Ok(i) => i,
            Err(_) => -1
        };
        //return ret;
    }

    pub fn get_url (&mut self) -> String{
        let url:String = match self.conn.rpop("url_queue"){
            Ok(x) => x,
            Err(_) => "".to_string()
        };
        return url;
    }


}
