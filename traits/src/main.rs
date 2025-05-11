mod basket;
use basket::Basket;

fn main() {
    let mut  basket = Basket::new(String::from("Water"));
    println!("{:#?}",basket.get());
    println!("{:#?}",basket.is_empty());
    basket.put("lava".to_string());
    println!("{:#?}",basket.get());
}
