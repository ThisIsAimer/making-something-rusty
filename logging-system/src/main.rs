use std::fs; // import io::Error for Error::other

fn extract_errors(text: &str) -> Vec<&str> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for lines in split_text {
        if lines.starts_with("ERROR") {
            results.push(lines);
        }
    }

    results
}

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(text) => {
            let error_lines = extract_errors(&text);
            println!("{:#?}", error_lines);
        }
        Err(error) => {
            println!("error found: {}", error)
        }
    }

    // match devide(3.0, 0.0){
    //     Ok(value) => {
    //         println!("the value is: {}",value)
    //     }
    //     Err(value) =>{
    //         println!("{value}")
    //     }
    // }

    // match validate_email("abca@abcd.com".to_string()) {

    //     Ok(()) => { //or ..
    //         println!("mail is valid")
    //     }

    //     Err(value) => {
    //         println!("{}",value)
    //     }

    // }
}

// fn validate_email(email:String) -> Result<(), Error>{
//     if email.contains("@"){
//         Ok(()) //() is an empty touple
//     }
//     else{
//         Err(Error::other("email must contain a @"))
//     }
// }

// fn devide(a:f64,b:f64)-> Result<f64,Error>{
//     if b == 0.0 {
//         Err(Error::other("cant devide by zero"))
//     }
//     else{
//         Ok(a/b)
//     }

// }
