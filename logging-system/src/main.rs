use std::{io::Error, fs}; // import io::Error for Error::other

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

fn main() -> Result<(),Error> {
     let text = fs::read_to_string("logs.txt")?;//? is try notation
     let errors_text = extract_errors(&text);
     fs::write("errors.txt", errors_text.join("\n"))?;


     Ok(()) // gives error when ? fails. but doesnt give OK value so, we state Ok

     /*or 
     let text = fs::read_to_string("logs.txt").expect("failed to read logs.txt");
     let errors_text = extract_errors(&text);
     fs::write("errors.txt", errors_text.join("\n")).expect("failed to write errors");
      */
}