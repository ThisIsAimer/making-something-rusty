use super::media::Media;

#[derive(Debug)]
pub struct Catalog{
    pub items : Vec<Media>
}

impl Catalog{
    pub fn new() -> Self{
        Catalog {items : vec![]}
    }
    pub fn add(&mut self, item: Media) {
        self.items.push(item);
    }

    pub fn get_item(&self,index:usize) -> Option<&Media>{ //Option<&Media>
        if self.items.len() > index{
            Some(&self.items[index])
        }else{
            None
        }
    }
}