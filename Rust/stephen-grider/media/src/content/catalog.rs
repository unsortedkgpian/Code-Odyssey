// mod media;
// use Media;

// mod content::media::Media;
// use media::Media;

use super::media::Media;

pub enum MightHaveAValue<'a> {
    ThereIsAValue(&'a Media),
    NoValueAvailable,
}


#[derive(Debug)]
pub struct Catalog {
    pub items:Vec<Media>
}

impl Catalog {
    pub fn new() -> Self {
        Catalog {items: vec![]}
    }

    pub fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    pub fn get_by_Index(&self , index:usize) -> MightHaveAValue{
        if self.items.len() > index {
            MightHaveAValue::ThereIsAValue(&self.items[index])
        }else{
            MightHaveAValue::NoValueAvailable
        }
        //&self.items[index]
    }

    pub fn get_by_index(&self, index:usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        }else{
            None
        }
    }
}

