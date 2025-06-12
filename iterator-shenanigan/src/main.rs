use std::vec;

fn print_element(elements: &[String]){// we can use vector slice type &[String] to use a portion of a vector
    elements.iter().map(|ele| format!("{}, {}",ele,ele)).for_each(|element| println!("{}",element));
}

fn shorten_strings(elements : &mut Vec<String>){
    elements.iter_mut().for_each(|element| element.truncate(1));
}

fn upper_case(element : &Vec<String>) -> Vec<String>{
    //element.iter().map(|element| element.to_uppercase()).collect()
    element.iter().map(|element| element.to_uppercase()).collect()
    // collect::<Vec<String>>() or <Vec<_>> _ tells rust to figure it out
}

fn move_vec(element: Vec<String>) -> Vec<String>{
    let mut new_ele = vec![];
    element.into_iter().for_each(|element| new_ele.push(element));
    new_ele
}

fn explode(element: &Vec<String>) -> Vec<Vec<String>>{
    element.iter().map(|element| element.chars().map(|char| char.to_string()).collect()).collect()
}

fn find_word(element : &Vec<String>, search : &str, placeholder: &str) -> String{
    element.iter().find(|element| element.contains(search)).map_or(placeholder.to_string(),|e| e.to_string())
    //in map_or default must be the none statement
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

    let space_objects = move_vec(planets);
    print_element(&space_objects);
    println!("{:#?}", explode(&colors));

    println!("{:#?}",find_word(&colors, "eue", "orange"));

}