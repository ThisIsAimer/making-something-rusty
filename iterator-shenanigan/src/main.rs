fn print_element(elements: &[String]){// we can use vector slice type &[String] to use a portion of a vector
    elements.iter().map(|ele| format!("{}, {}",ele,ele)).for_each(|element| println!("{}",element));
}

fn main() {
    let colors = vec![
        "red".to_string(),
        "blue".to_string(),
        "green".to_string(),
    ];

    print_element(&colors[1..3]);
}