use std::{fs, io::Error};// import io::Error for Error::other

fn main() {
    let text = fs::read_to_string("logs.txt");
    println!("{:#?}",text);

    match devide(3.0, 0.0){
        Ok(value) => {
            print!("{}",value)
        }
        Err(value) =>{
            print!("{value}")
        }
    }
    
}

fn devide(a:f64,b:f64)-> Result<f64,Error>{
    if b == 0.0 {
        Err(Error::other("cant devide by zero"))
    }
    else{
        Ok(a/b)
    }

}
