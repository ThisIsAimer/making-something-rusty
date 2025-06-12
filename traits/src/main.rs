mod basket;
mod stack;
mod container;
use basket::Basket;
use stack::Stack;
use container::Container;// necessart now

fn add_string<T:Container<String>>(element: &mut T, string: String){
    element.put(string);
}

fn main() {
    let mut  basket = Basket::new(String::from("Water"));
    println!("{:#?}",basket.get());
    println!("{:#?}",basket.is_empty());
    basket.put("lava".to_string());
    println!("{:#?}",basket.get());


    let mut stack = Stack::new(vec!["water".to_string()]);
    let  nums = Stack::new(vec![1,2,3]);
    add_string(&mut stack, String::from("lava"));
    println!("{:#?}",stack);
    println!("{:#?}",nums);
}
