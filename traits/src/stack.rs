#[derive(Debug)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new(items: Vec<T>) -> Self {
        Stack { items: items }
    }

    pub fn get(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn put(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
