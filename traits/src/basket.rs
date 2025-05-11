#[derive(Debug)]
pub struct Basket{
    item : Option<String>
}

impl Basket {
    pub fn new(item :String) -> Self{
        Basket { item: Some(item) }
    }

    pub fn get(&mut self) -> Option<String>{
        self.item.take()
    }

    pub fn put(&mut self, item: String){
        self.item = Some(item)
    }

    pub fn is_empty(&self) -> bool{
        self.item.is_none()
    }
}