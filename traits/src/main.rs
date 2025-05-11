mod basket;
mod stack;
mod container;
use basket::Basket;
use stack::Stack;
use crate::container::Container;// necessart now

fn main() {
    let mut  basket = Basket::new(String::from("Water"));
    println!("{:#?}",basket.get());
    println!("{:#?}",basket.is_empty());
    basket.put("lava".to_string());
    println!("{:#?}",basket.get());


    let mut stack = Stack::new(vec!["water".to_string()]);
    let mut nums = Stack::new(vec![1,2,3]);
}
