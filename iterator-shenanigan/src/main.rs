fn print_element(elements: &Vec<String>){
    elements.iter().map(|ele| format!("{}, {}",ele,ele)).for_each(|element| println!("{}",element));
}

fn main() {
    let colors = vec![
        "red".to_string(),
        "blue".to_string(),
        "green".to_string(),
    ];

    print_element(&colors);
}