use std::str;
use std::string::String;
use std::collections::hash_map::HashMap;
use htmlstream;

pub  struct HtmlParser {
    hrefs: Vec<String>,
    videos: Vec<String>,
    imgs: Vec<String>,
}

impl HtmlParser{
   pub  fn new(content: String) -> HtmlParser {
        let mut hrefs :Vec<String> = Vec::new();
        let mut videos :Vec<String> = Vec::new();
        let mut imgs :Vec<String> = Vec::new();
        for (pos, tag) in htmlstream::tag_iter(&content) {
            if tag.name == "a" {
                for (pos, attr) in htmlstream::attr_iter(&tag.attributes) {
                    if attr.name == "href" {
                        hrefs.push(attr.value);
                    }
                }
            }
            if tag.name == "video" {
                for (pos, attr) in htmlstream::attr_iter(&tag.attributes) {
                    if attr.name == "src" {
                        videos.push(attr.value);
                    }
                }
            }
            if tag.name == "img" {
                for (pos, attr) in htmlstream::attr_iter(&tag.attributes) {
                    if attr.name == "src" {
                        imgs.push(attr.value);
                    }
                }
            }
        }
        return HtmlParser{hrefs:hrefs,videos:videos,imgs:imgs};
    }

    pub fn get_a(&mut self) -> Vec<String>{
        return self.hrefs.clone();
    }
    pub fn get_img(&mut self) -> Vec<String>{
        return self.imgs.clone();
    }
    pub fn get_video(&mut self) -> Vec<String>{
        return self.videos.clone();
    }
}
