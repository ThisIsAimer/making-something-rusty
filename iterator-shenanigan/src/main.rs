fn print_element(elements: &[String]){// we can use vector slice type &[String] to use a portion of a vector
    elements.iter().map(|ele| format!("{}, {}",ele,ele)).for_each(|element| println!("{}",element));
}

fn shorten_strings(elements : &mut Vec<String>){
    elements.iter_mut().for_each(|element| element.truncate(1));
}

fn upper_case(element : &Vec<String>) -> Vec<String>{
    //element.iter().map(|element| element.to_uppercase()).collect()
    element.iter().map(|element| element.to_uppercase()).collect()
}

fn main() {
    let  colors = vec![
        "red".to_string(),
        "blue".to_string(),
        "green".to_string(),
    ];

    let mut planets = vec![
        "earth".to_string(),
        "mars".to_string(),
        "venus".to_string(),
    ];

    print_element(&colors);
    shorten_strings(&mut planets);
    print_element(&planets);
    println!("{:#?}",upper_case(&colors));

}