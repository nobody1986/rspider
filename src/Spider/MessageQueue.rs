//use redis;
//use redis::{Commands, PipelineCommands, transaction};
use rusqlite;
use rusqlite::Connection;

use std::str;
use std::string::String;
use std::collections::hash_map::HashMap;


pub struct MessageQueue{
    conn: Connection
}

impl MessageQueue{
    pub fn new ()  ->MessageQueue {
        /*
        let client =  match redis::Client::open("redis://127.0.0.1/") {
            Ok(x) => x,
            Err(_) => panic!("redis connect error")
        };

            let con = match client.get_connection() {
                Ok(x) => x,
            Err(_) => panic!("redis connect error")
            };
            */
            let con = Connection::open_in_memory().unwrap();
            con.execute("CREATE TABLE queue (
                  id              INTEGER PRIMARY KEY,
                  key            TEXT NOT NULL,
                  value            BLOB
                  )", &[]).unwrap();
            return MessageQueue{conn: con};

    }

    pub fn add_to_queue (&mut self ,url: &mut String,level: i32){
        if self.url_exists(url) < 0 {
            //let ret:Result<i32,_> =  self.conn.hset("visited",url.clone(),level );
            //let ret:Result<String,_> = self.conn.lpush("url_queue",url.clone());
            self.conn.execute("INSERT INTO queue (key, value)
                  VALUES ($1, $2)",
                 &[&"visited", url.clone()]).unwrap();
            self.conn.execute("INSERT INTO queue (key, value)
                VALUES ($1, $2)",
                &[&"url_queue", url.clone()]).unwrap();
        }
    }

    pub fn url_exists (&mut self ,url:  &mut  String) -> i32{
        //let ret:Result<i32,_> = self.conn.hget("visited",url.clone());
        //println!("{:?}",ret );
        let mut stmt = self.conn.prepare("SELECT count(1) FROM queue where key=:key").unwrap();
        let mut visited = stmt.query_named(&[(":key", &"visited")]).unwrap();
        let ret:i32 = -1;
        while let Some(result_row) = visited.next() {
                let row = try!(result_row);
            //println!("Found person {:?}", person.unwrap());
            if row.get(0) > 0{
                ret = 1;
            }
        }
        return ret;
        /*
        return match ret {
            Ok(i) => i,
            Err(_) => -1
        };
        */
        //return ret;
    }

    pub fn get_url (&mut self) -> String {

        /*let url:String = match self.conn.rpop("url_queue"){
            Ok(x) => x,
            Err(_) => "".to_string()
        };
        return url;*/
        let mut stmt = self.conn.prepare("SELECT id,value FROM queue where key=:key order by id asc limit 0,1").unwrap();
        let mut queue = stmt.query_named(&[(":key", &"url_queue")]).unwrap();
        while let Some(result_row) = queue.next() {
                let row = try!(result_row);
            //println!("Found person {:?}", person.unwrap());
                self.conn.execute("delete from queue  where id=$1",
                     &[&row.get(0)]).unwrap();
                return row.get(1);
        }
        return "".to_string();
    }


}
