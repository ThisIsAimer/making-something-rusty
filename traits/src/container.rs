pub trait Container<T>{

    fn get(&mut self) -> Option<T>;
    fn put(&mut self, items: T );
    fn is_empty(&self) -> bool;
}