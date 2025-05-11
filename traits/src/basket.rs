use super::container::Container;

#[derive(Debug)]
//basket can store multiple type of items, but when an item is created with a type, it will only hold that type of item
pub struct Basket<T>{
    item : Option<T>
}

impl<T> Basket<T> {
    pub fn new(item :T) -> Self{
        Basket { item: Some(item) }
    }

}

impl<T> Container<T> for Basket<T>{
    fn get(&mut self) -> Option<T>{
        self.item.take()
    }

    fn put(&mut self, item: T){
        self.item = Some(item)
    }

    fn is_empty(&self) -> bool{
        self.item.is_none()
    }
}