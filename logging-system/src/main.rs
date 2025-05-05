use std::{fs, io::Error};// import io::Error for Error::other

fn main() {
    match fs::read_to_string("logs.txt") {

        Ok(text)=> {println!("text that was read:-\n{}",text)}
        Err(error) =>{println!("error found: {}", error)}
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
