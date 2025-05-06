use std::fs; // import io::Error for Error::other

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for lines in split_text {
        if lines.starts_with("ERROR") {
            results.push(lines.to_string());
        }
    }

    results
}

fn main() {
    let mut error_lines = vec![];
    match fs::read_to_string("logs.txt") {
        Ok(text) => {
            error_lines = extract_errors(&text);
        }
        Err(error) => {
            println!("error found: {}", error);
        }
    }
    println!("{:#?}", error_lines);

    match fs::write("errors.txt", error_lines.join("\n")) {
        Ok(()) =>{
            println!("writing was successfull")
        }
        Err(error) => {
            println!("error: {}", error);
        }
    }
}