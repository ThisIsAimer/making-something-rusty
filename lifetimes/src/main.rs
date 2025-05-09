/* when a function has two or more references as parameters and returns a 
reference, rust will assume that the reference points to one of the paramers
as <'a> and &'a are supposed to be put to notify rust that its that parameter*/
fn next_lang<'a>(elements : &'a[String], value: &str)-> &'a str{
    let mut found = false;

    for element in elements{
        if element == value{
            found = true;
        }
        if found{
            return element;
        }
    }

    elements.last().unwrap()
}

//only recieves one reference
fn last_element(elements: &[String]) -> & str{
    elements.last().unwrap()
}


fn main() {
    let lang = vec![
        "python".to_string(),
        "Rust".to_string(),
        "Go".to_string()
    ];

    let nextlang = next_lang(&lang, "Rust");
    println!("{:#?}",nextlang);
    println!("{:#?}",last_element(&lang));
}
